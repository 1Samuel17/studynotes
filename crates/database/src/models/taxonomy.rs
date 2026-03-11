use sea_orm::entity::prelude::*;

// Taxonomy module defining categories and tags for notes

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Category {
    #[sea_orm(string_value = "Rust Standard Library")]
    RustStandardLibrary,
    #[sea_orm(string_value = "Crates.io")]
    CratesIo,
    #[sea_orm(string_value = "LeetCode")]
    LeetCode,
    #[sea_orm(string_value = "Coursera")]
    Coursera,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Tag {
    #[sea_orm(string_value = "Important")]
    Important,
    #[sea_orm(string_value = "Async")]
    Async,
    #[sea_orm(string_value = "Web Development")]
    WebDevelopment,
    #[sea_orm(string_value = "Data Structures")]
    DataStructures,
    #[sea_orm(string_value = "Algorithms")]
    Algorithms,
    #[sea_orm(string_value = "Machine Learning")]
    MachineLearning,
    #[sea_orm(string_value = "Databases")]
    Databases,
    #[sea_orm(string_value = "System Programming")]
    SystemProgramming,
    #[sea_orm(string_value = "Networking")]
    Networking,
    #[sea_orm(string_value = "Security")]
    Security,
    #[sea_orm(string_value = "Operating Systems")]
    OperatingSystems,
    #[sea_orm(string_value = "Compilers")]
    Compilers,
    #[sea_orm(string_value = "Concurrency")]
    Concurrency,
    #[sea_orm(string_value = "Testing")]
    Testing,
    #[sea_orm(string_value = "Debugging")]
    Debugging,
    #[sea_orm(string_value = "Performance Optimization")]
    PerformanceOptimization,
    #[sea_orm(string_value = "Code Organization")]
    CodeOrganization,
    #[sea_orm(string_value = "Best Practices")]
    BestPractices,
    #[sea_orm(string_value = "Design Patterns")]
    DesignPatterns,
    #[sea_orm(string_value = "Software Architecture")]
    SoftwareArchitecture,
    #[sea_orm(string_value = "Version Control")]
    VersionControl,
    #[sea_orm(string_value = "Documentation")]
    Documentation,
    #[sea_orm(string_value = "Medical Software")]
    MedicalSoftware,
    #[sea_orm(string_value = "Medical Industry")]
    MedicalIndustry,
}
