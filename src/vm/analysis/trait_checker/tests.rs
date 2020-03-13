use vm::ast::{parse, build_ast};
use vm::ast::errors::ParseErrors;
use vm::analysis::errors::CheckErrors;
use vm::analysis::{AnalysisDatabase, contract_interface_builder::build_contract_interface};
use vm::database::MemoryBackingStore;
use vm::analysis::mem_type_check;
use vm::analysis::type_check;
use vm::types::{TypeSignature, QualifiedContractIdentifier};

#[test]
fn test_dynamic_dispatch_by_defining_trait() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let target_contract_src =
        "(define-public (get-1 (x uint)) (ok u1))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap();
}


#[test]
fn test_incomplete_impl_trait_1() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))
            (get-2 (uint) (response uint uint))
            (get-3 (uint) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))";
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    let err = db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::BadTraitImplementation(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_incomplete_impl_trait_2() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))
            (get-2 (uint) (response uint uint))
            (get-3 (uint) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))
        (define-public (get-2 (x uint)) (ok u1))";    
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    let err = db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::BadTraitImplementation(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_impl_trait_arg_admission_1() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 ((list 10 uint)) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x (list 5 uint))) (ok u1))";
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    let err = db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::BadTraitImplementation(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_impl_trait_arg_admission_2() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 ((list 5 uint)) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x (list 15 uint))) (ok u1))";
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap();
}

#[test]
fn test_impl_trait_arg_admission_3() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 ((list 5 uint)) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x (list 5 uint))) (ok u1))";
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap();
}

#[test]
fn test_complete_impl_trait() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))
            (get-2 (uint) (response uint uint))
            (get-3 (uint) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))
        (define-public (get-2 (x uint)) (ok u1))
        (define-public (get-3 (x uint)) (ok u1))";
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap();
}

#[test]
fn test_complete_impl_trait_mixing_readonly() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))
            (get-2 (uint) (response uint uint))
            (get-3 (uint) (response uint uint))))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))
        (define-read-only (get-2 (x uint)) (ok u1))
        (define-read-only (get-3 (x uint)) (ok u1))";
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true)
    }).unwrap();
}

#[test]
fn test_get_trait_reference_from_tuple() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (wrapped-contract (tuple (contract <trait-1>)))) 
            (contract-call? (get contract wrapped-contract) get-1 u0))";
    let target_contract_src =
        "(define-public (get-1 (x uint)) (ok u1))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::ContractCallExpectName => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_by_defining_and_impl_trait() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (impl-trait .dispatching-contract.trait-1)
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))
        (define-public (get-1 (x uint)) (ok u1))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::TraitReferenceUnknown(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_define_map_storing_trait_references() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-map kv-store ((key uint)) ((value <trait-1>)))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let err = build_ast(&dispatching_contract_id, dispatching_contract_src).unwrap_err();

    match err.err {
        ParseErrors::TraitReferenceNotAllowed => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_cycle_in_traits_1_contract() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (<trait-2>) (response uint uint))))
        (define-trait trait-2 (
            (get-2 (<trait-1>) (response uint uint))))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let err = build_ast(&dispatching_contract_id, dispatching_contract_src).unwrap_err();
    match err.err {
        ParseErrors::CircularReference(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_cycle_in_traits_2_contracts() {
    let dispatching_contract_src =
        "(use-trait trait-2 .target-contract.trait-2)
        (define-trait trait-1 (
            (get-1 (<trait-2>) (response uint uint))))";
    let target_contract_src =
        "(use-trait trait-1 .dispatching-contract.trait-1)
        (define-trait trait-2 (
            (get-2 (<trait-1>) (response uint uint))))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::NoSuchContract(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_unknown_method() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-2 u0))";
    let target_contract_src =
        "(define-public (get-1 (x uint)) (ok u1))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::TraitMethodUnknown(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_nested_literal_implicitly_compliant() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let nested_target_contract_src =
        "(define-public (get-1 (x uint)) (ok u1))";
    let target_contract_src =
        "(define-public (get-1 (x uint)) (contract-call? .dispatching-contract wrapped-get-1 .nested-target-contract))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();
    let nested_target_contract_id = QualifiedContractIdentifier::local("nested-target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut nested_target_contract = parse(&nested_target_contract_id, nested_target_contract_src).unwrap();

    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&nested_target_contract_id, &mut nested_target_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap();
}

#[test]
fn test_passing_trait_reference_instances() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (internal-get-1 contract))
        (define-public (internal-get-1 (contract <trait-1>))
            (ok u1))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)
    }).unwrap();
}

#[test]
fn test_dynamic_dispatch_collision_trait() {
    let contract_defining_trait_src = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract_src =
        "(use-trait trait-1 .contract-defining-trait.trait-1)
        (define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";

    let contract_defining_trait_id = QualifiedContractIdentifier::local("contract-defining-trait").unwrap();
    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let _contract_defining_trait = parse(&contract_defining_trait_id, contract_defining_trait_src).unwrap();
    let err = build_ast(&dispatching_contract_id, dispatching_contract_src).unwrap_err();
    match err.err {
        ParseErrors::NameAlreadyUsed(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_collision_defined_trait() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-trait trait-1 (
            (get-1 (int) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let err = build_ast(&dispatching_contract_id, dispatching_contract_src).unwrap_err();
    match err.err {
        ParseErrors::NameAlreadyUsed(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_collision_imported_trait() {
    let contract_defining_trait_src = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
         (define-trait trait-2 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract_src =
        "(use-trait trait-1 .contract-defining-trait.trait-1)
        (use-trait trait-1 .contract-defining-trait.trait-2)
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";

    let contract_defining_trait_id = QualifiedContractIdentifier::local("contract-defining-trait").unwrap();
    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();

    let _contract_defining_trait = parse(&contract_defining_trait_id, contract_defining_trait_src).unwrap();
    let err = build_ast(&dispatching_contract_id, dispatching_contract_src).unwrap_err();
    match err.err {
        ParseErrors::NameAlreadyUsed(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_importing_non_existant_trait() {
    let contract_defining_trait_src = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract_src =
        "(use-trait trait-1 .contract-defining-trait.trait-2)
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let target_contract_src =
        "(impl-trait .contract-defining-trait.trait-2)
        (define-public (get-1 (x uint)) (ok u1))";

    let contract_defining_trait_id = QualifiedContractIdentifier::local("contract-defining-trait").unwrap();
    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut contract_defining_trait = parse(&contract_defining_trait_id, contract_defining_trait_src).unwrap();
    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&contract_defining_trait_id, &mut contract_defining_trait, db, true)?;
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::TraitReferenceUnknown(_) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_importing_trait() {
    let contract_defining_trait_src = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract_src =
        "(use-trait trait-1 .contract-defining-trait.trait-1)
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let target_contract_src =
        "(impl-trait .contract-defining-trait.trait-1)
        (define-public (get-1 (x uint)) (ok u1))";

    let contract_defining_trait_id = QualifiedContractIdentifier::local("contract-defining-trait").unwrap();
    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut contract_defining_trait = parse(&contract_defining_trait_id, contract_defining_trait_src).unwrap();
    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    db.execute(|db| {
        type_check(&contract_defining_trait_id, &mut contract_defining_trait, db, true)?;
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap();
}

#[test]
fn test_dynamic_dispatch_including_nested_trait() {
    let contract_defining_nested_trait_src = 
    "(define-trait trait-a (
        (get-a (uint) (response uint uint))))";
    let contract_defining_trait_src = 
        "(use-trait trait-Z .contract-defining-nested-trait.trait-a)
        (define-trait trait-1 (
            (get-1 (<trait-Z>) (response uint uint))))";
    let dispatching_contract_src =
        "(use-trait trait-Y .contract-defining-nested-trait.trait-a)
         (use-trait trait-X .contract-defining-trait.trait-1)
         (define-public (wrapped-get-1 (contract <trait-X>) (nested-contract <trait-Y>)) 
            (contract-call? contract get-1 nested-contract))";
    let target_contract_src =
        "(use-trait trait-X .contract-defining-nested-trait.trait-a)
        (define-public (get-1 (nested-contract <trait-X>))
            (contract-call? nested-contract get-a u0))";
    let target_nested_contract_src =
        "(define-public (get-a (x uint)) (ok u99))";

    let contract_defining_trait_id = QualifiedContractIdentifier::local("contract-defining-trait").unwrap();
    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();
    let contract_defining_nested_trait_id = QualifiedContractIdentifier::local("contract-defining-nested-trait").unwrap();
    let target_nested_contract_id = QualifiedContractIdentifier::local("target-nested-contract").unwrap();

    let mut contract_defining_trait = parse(&contract_defining_trait_id, contract_defining_trait_src).unwrap();
    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut contract_defining_nested_trait = parse(&contract_defining_nested_trait_id, contract_defining_nested_trait_src).unwrap();
    let mut target_nested_contract = parse(&target_nested_contract_id, target_nested_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    db.execute(|db| {
        type_check(&contract_defining_nested_trait_id, &mut contract_defining_nested_trait, db, true)?;
        type_check(&contract_defining_trait_id, &mut contract_defining_trait, db, true)?;
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)?;
        type_check(&target_nested_contract_id, &mut target_nested_contract, db, true)
    }).unwrap();
    
}

#[test]
fn test_dynamic_dispatch_including_wrong_nested_trait() {
    let contract_defining_nested_trait_src = 
    "(define-trait trait-a (
        (get-a (uint) (response uint uint))))";
    let contract_defining_trait_src = 
        "(use-trait trait-a .contract-defining-nested-trait.trait-a)
        (define-trait trait-1 (
            (get-1 (<trait-a>) (response uint uint))))";
    let dispatching_contract_src =
        "(use-trait trait-1 .contract-defining-trait.trait-1)
         (use-trait trait-a .contract-defining-trait.trait-a)
         (define-public (wrapped-get-1 (contract <trait-1>) (nested-contract <trait-1>)) 
            (contract-call? contract get-1 nested-contract))";
    let target_contract_src =
        "(use-trait trait-a .contract-defining-nested-trait.trait-a)
        (define-public (get-1 (nested-contract <trait-a>))
            (contract-call? nested-contract get-a u0))";
    let target_nested_contract_src =
        "(define-public (get-a (x uint)) (ok u99))";

    let contract_defining_trait_id = QualifiedContractIdentifier::local("contract-defining-trait").unwrap();
    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();
    let contract_defining_nested_trait_id = QualifiedContractIdentifier::local("contract-defining-nested-trait").unwrap();
    let target_nested_contract_id = QualifiedContractIdentifier::local("target-nested-contract").unwrap();

    let mut contract_defining_trait = parse(&contract_defining_trait_id, contract_defining_trait_src).unwrap();
    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut contract_defining_nested_trait = parse(&contract_defining_nested_trait_id, contract_defining_nested_trait_src).unwrap();
    let mut target_nested_contract = parse(&target_nested_contract_id, target_nested_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&contract_defining_nested_trait_id, &mut contract_defining_nested_trait, db, true)?;
        type_check(&contract_defining_trait_id, &mut contract_defining_trait, db, true)?;
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)?;
        type_check(&target_nested_contract_id, &mut target_nested_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::TypeError(TypeSignature::TraitReferenceType(_), TypeSignature::TraitReferenceType(_)) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_mismatched_args() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (int) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let target_contract_src =
        "(impl-trait .dispatching-contract.trait-1)
        (define-public (get-1 (x uint)) (ok u1))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::TypeError(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_dynamic_dispatch_mismatched_returns() {
    let dispatching_contract_src =
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let target_contract_src =
        "(impl-trait .dispatching-contract.trait-1)
        (define-public (get-1 (x uint)) (ok \"buffer\"))";

    let dispatching_contract_id = QualifiedContractIdentifier::local("dispatching-contract").unwrap();
    let target_contract_id = QualifiedContractIdentifier::local("target-contract").unwrap();

    let mut dispatching_contract = parse(&dispatching_contract_id, dispatching_contract_src).unwrap();
    let mut target_contract = parse(&target_contract_id, target_contract_src).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    let err = db.execute(|db| {
        type_check(&dispatching_contract_id, &mut dispatching_contract, db, true)?;
        type_check(&target_contract_id, &mut target_contract, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::BadTraitImplementation(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_bad_call_with_trait() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract =
        "(use-trait trait-1 .defun.trait-1)
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))";
    let caller_contract = // Should error.
        "(define-constant contract .implem)
         (define-public (foo-bar)
           (contract-call? .dispatch wrapped-get-1 contract))"; 
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let disp_contract_id = QualifiedContractIdentifier::local("dispatch").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let call_contract_id = QualifiedContractIdentifier::local("call").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c2 = parse(&disp_contract_id, dispatching_contract).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut c4 = parse(&call_contract_id, caller_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();
    let err = db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&disp_contract_id, &mut c2, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true).unwrap();
        type_check(&call_contract_id, &mut c4, db, true)
    }).unwrap_err();
    match err.err {
        CheckErrors::TypeError(_, _) => {},
        _ => {
            panic!("{:?}", err)
        }
    }
}

#[test]
fn test_good_call_with_trait() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract =
        "(use-trait trait-1 .defun.trait-1)
        (define-public (wrapped-get-1 (contract <trait-1>)) 
            (contract-call? contract get-1 u0))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))";
    let caller_contract =
        "(define-public (foo-bar)
           (contract-call? .dispatch wrapped-get-1 .implem))"; 
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let disp_contract_id = QualifiedContractIdentifier::local("dispatch").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let call_contract_id = QualifiedContractIdentifier::local("call").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c2 = parse(&disp_contract_id, dispatching_contract).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut c4 = parse(&call_contract_id, caller_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    println!("c4: {:?}", c4);

    db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&disp_contract_id, &mut c2, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true).unwrap();
        type_check(&call_contract_id, &mut c4, db, true)
    }).unwrap();
}

#[test]
fn test_good_call_2_with_trait() {
    let contract_defining_trait = 
        "(define-trait trait-1 (
            (get-1 (uint) (response uint uint))))";
    let dispatching_contract =
        "(use-trait trait-2 .defun.trait-1)
        (define-public (wrapped-get-1 (contract <trait-2>)) 
            (contract-call? contract get-1 u0))";
    let impl_contract = 
        "(impl-trait .defun.trait-1)
        (define-public (get-1 (x uint)) (ok u1))";
    let caller_contract =
        "(use-trait trait-2 .defun.trait-1)
         (define-public (foo-bar (contract <trait-2>))
           (contract-call? .dispatch wrapped-get-1 contract))"; 
    let def_contract_id = QualifiedContractIdentifier::local("defun").unwrap();
    let disp_contract_id = QualifiedContractIdentifier::local("dispatch").unwrap();
    let impl_contract_id = QualifiedContractIdentifier::local("implem").unwrap();
    let call_contract_id = QualifiedContractIdentifier::local("call").unwrap();
    let mut c1 = parse(&def_contract_id, contract_defining_trait).unwrap();
    let mut c2 = parse(&disp_contract_id, dispatching_contract).unwrap();
    let mut c3 = parse(&impl_contract_id, impl_contract).unwrap();
    let mut c4 = parse(&call_contract_id, caller_contract).unwrap();
    let mut marf = MemoryBackingStore::new();
    let mut db = marf.as_analysis_db();

    println!("c4: {:?}", c4);

    db.execute(|db| {
        type_check(&def_contract_id, &mut c1, db, true).unwrap();
        type_check(&disp_contract_id, &mut c2, db, true).unwrap();
        type_check(&impl_contract_id, &mut c3, db, true).unwrap();
        type_check(&call_contract_id, &mut c4, db, true)
    }).unwrap();
}
