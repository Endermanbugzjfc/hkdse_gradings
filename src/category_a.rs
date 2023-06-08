#[derive(Display)]
enum Gradings {
    Unclassified,
    Unsatisfactory,
    Marginal,
    Average,
    Satisfactory,
    Good,
    Outstanding,
    Excellent,
}

impl Gradings {
    /// Reminder: please convert your argument to uppercase
    /// before passing to this function.
    fn from_indicator(indicator: &str) -> Option<Self> {
        Some(match indicator {
            "U" => Self::Unclassified,
            "1" => Self::Unsatisfactory,
            "2" => Self::Marginal,
            "3" => Self::Average,
            "4" => Self::Satisfactory,
            "5" => Self::Good,
            "5*" => Self::Outstanding,
            "5**" => Self::Excellant,
            _ => return None,
        })
    }

    fn to_indicator(self) -> &'static str {
        match self {
            Self::Unclassified => "U",
            Self::Unsatisfactory => "1",
            Self::Marginal => "2",
            Self::Average => "3",
            Self::Satisfactory => "4",
            Self::Good => "5",
            Self::Outstanding => "5*",
            Self::Excellant => "5**",
        }
    }
}

enum CSDGradings {
    Unattained,
    Attained,
}

impl CSDGradings {
    /// Reminder: please convert your argument to uppercase
    /// before passing to this function.
    fn from_indicator(indicator: &str) -> Option<Self> {
        Some(match indicator {
            "U" => Self::Unattained,
            "A" => Self::Attained,
            _ => return None,
        })
    }

    fn to_indicator(self) -> &'static str {
        match self {
            Self::Unattained => "U",
            Self::Attained => "A",
        }
    }
}