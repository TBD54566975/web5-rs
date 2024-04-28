pub enum RegisteredDidType {
  Discoverable,
  Organization,
  GovernmentOrganization,
  Corporation,
  LocalBusiness,
  SoftwarePackage,
  WebApp,
  FinancialInstitution,
}

impl RegisteredDidType {
  fn name(&self) -> &'static str {
      match self {
          RegisteredDidType::Discoverable => "Discoverable",
          RegisteredDidType::Organization => "Organization",
          RegisteredDidType::GovernmentOrganization => "Government Organization",
          RegisteredDidType::Corporation => "Corporation",
          RegisteredDidType::LocalBusiness => "Local Business",
          RegisteredDidType::SoftwarePackage => "Software Package",
          RegisteredDidType::WebApp => "Web App",
          RegisteredDidType::FinancialInstitution => "Financial Institution",
      }
  }

  fn id(&self) -> i32 {
      match self {
          RegisteredDidType::Discoverable => 0,
          RegisteredDidType::Organization => 1,
          RegisteredDidType::GovernmentOrganization => 2,
          RegisteredDidType::Corporation => 3,
          RegisteredDidType::LocalBusiness => 4,
          RegisteredDidType::SoftwarePackage => 5,
          RegisteredDidType::WebApp => 6,
          RegisteredDidType::FinancialInstitution => 7,
      }
  }
}