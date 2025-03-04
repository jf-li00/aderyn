use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::{NodeID, NodeType},
    capture,
    context::{
        browser::GetClosestAncestorOfTypeX,
        workspace_context::{ASTNode, WorkspaceContext},
    },
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UnsafeERC721MintDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UnsafeERC721MintDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers() {
            // if source_unit has any ImportDirectives with absolute_path containing "openzeppelin"
            // call identifier.accept(self)
            if let Some(ASTNode::SourceUnit(source_unit)) =
                identifier.closest_ancestor_of_type(context, NodeType::SourceUnit)
            {
                let import_directives = source_unit.import_directives();
                if import_directives.iter().any(|directive| {
                    directive
                        .absolute_path
                        .as_ref()
                        .map_or(false, |path| path.contains("openzeppelin"))
                }) && identifier.name == "_mint"
                {
                    let this_contract_definition = identifier
                        .closest_ancestor_of_type(context, NodeType::ContractDefinition)
                        .unwrap();
                    if let ASTNode::ContractDefinition(contract_definition) =
                        this_contract_definition
                    {
                        for base_contract in contract_definition.base_contracts.iter() {
                            if base_contract.base_name.name.contains("ERC721") {
                                capture!(self, context, identifier);
                            }
                        }
                    }
                }
            }
        }
        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Using `ERC721::_mint()` can be dangerous")
    }

    fn description(&self) -> String {
        String::from(
            "Using `ERC721::_mint()` can mint ERC721 tokens to addresses which don't support ERC721 tokens. Use `_safeMint()` instead of `_mint()` for ERC721.",
        )
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UnsafeOzERC721Mint)
    }
}

#[cfg(test)]
mod unsafe_erc721_mint_tests {
    use crate::detect::{
        detector::{detector_test_helpers::load_contract, IssueDetector},
        low::UnsafeERC721MintDetector,
    };

    #[test]
    fn test_unsafe_erc721_mint_detector() {
        let context = load_contract(
            "../tests/contract-playground/out/UnsafeERC721Mint.sol/UnsafeERC721Mint.json",
        );

        let mut detector = UnsafeERC721MintDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct number of instance
        assert_eq!(detector.instances().len(), 1);
        // assert that the severity is Low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Using `ERC721::_mint()` can be dangerous")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Using `ERC721::_mint()` can mint ERC721 tokens to addresses which don't support ERC721 tokens. Use `_safeMint()` instead of `_mint()` for ERC721."
            )
        );
    }
}
