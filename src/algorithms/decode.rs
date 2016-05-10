use std::str::FromStr;
use rustc_serialize::{Decodable, Decoder};
use regex::Regex;
use super::{Algorithm, CellSelection, Bias, Scan};

impl Decodable for Algorithm {
    fn decode<D: Decoder>(d: &mut D) -> Result<Algorithm, D::Error> {
        let main_str = try!(d.read_str());
        let extract_fail = d.error("Failed to extract valid algorithm from the given algorithm string");
        let incorrect_format = d.error("The given algorithm string is not in the correct format");
        let mut result = Err(extract_fail);
        // we look for a single string or two strings separated by a comma or three strings separated
        // by two commas
        let main_pat = r"^(?P<algorithm>[^,]+)(:?,(?P<param>[^,]+))?(:?,(?P<value>[^,]+))?$";
        let main_re = Regex::new(main_pat).unwrap();
        // if the input string don't match, quit
        let caps = match main_re.captures(&main_str) {
            Some(caps) => caps,
            None => return Err(incorrect_format)
        };
        // to save time on typing we'll match partial algorithm names
        let algorithm_pat = format!("(?i)^{}.*$", caps.name("algorithm").unwrap_or(""));
        let algorithm_re = Regex::new(&algorithm_pat).unwrap();
        let param_pat = format!("(?i)^{}.*$", caps.name("param").unwrap_or(""));
        let param_re = Regex::new(&param_pat).unwrap();
        let value: Result<usize, _> = FromStr::from_str(caps.name("value").unwrap_or(""));
        if algorithm_re.is_match("PrimsAlgorithm") {
            result = Ok(Algorithm::PrimsAlgorithm);
        } else if algorithm_re.is_match("GrowingTree") {
            if param_re.is_match("Newest") {
                result = Ok(Algorithm::GrowingTree(CellSelection::Newest));
            } else if param_re.is_match("Oldest") {
                result = Ok(Algorithm::GrowingTree(CellSelection::Oldest));
            } else if param_re.is_match("Random") {
                result = Ok(Algorithm::GrowingTree(CellSelection::Random));
            }
            // all further options require some percentage as the last string
            if let Ok(n) = value {
                if param_re.is_match("NewestOldest") {
                    result = Ok(Algorithm::GrowingTree(CellSelection::NewestOldest(n)));
                } else if param_re.is_match("NewestRandom") {
                    result = Ok(Algorithm::GrowingTree(CellSelection::NewestRandom(n)));
                } else if param_re.is_match("OldestRandom") {
                    result = Ok(Algorithm::GrowingTree(CellSelection::OldestRandom(n)));
                }
            }
        } else if algorithm_re.is_match("BinaryTree") {
            if param_re.is_match("Northeast") {
                result = Ok(Algorithm::BinaryTree(Bias::Northeast));
            } else if param_re.is_match("Northwest") {
                result = Ok(Algorithm::BinaryTree(Bias::Northwest));
            } else if param_re.is_match("Southeast") {
                result = Ok(Algorithm::BinaryTree(Bias::Southeast));
            } else if param_re.is_match("Southwest") {
                result = Ok(Algorithm::BinaryTree(Bias::Southwest));
            }
       } else if algorithm_re.is_match("SidewinderAlgorithm") {
            if param_re.is_match("Horizontal") {
                result = Ok(Algorithm::SidewinderAlgorithm(Scan::Horizontal));
            } else if param_re.is_match("Vertical") {
                result = Ok(Algorithm::SidewinderAlgorithm(Scan::Vertical));
            }
        } else if algorithm_re.is_match("RecursiveBacktracking") {
            result = Ok(Algorithm::RecursiveBacktracking);
        } else if algorithm_re.is_match("EllersAlgorithm") {
            if param_re.is_match("Horizontal") {
                result = Ok(Algorithm::EllersAlgorithm(Scan::Horizontal));
            } else if param_re.is_match("Vertical") {
                result = Ok(Algorithm::EllersAlgorithm(Scan::Vertical));
            }
        } else if algorithm_re.is_match("KruskalsAlgorithm") {
            result = Ok(Algorithm::KruskalsAlgorithm);
        } else if algorithm_re.is_match("RecursiveDivision") {
            result = Ok(Algorithm::RecursiveDivision);
        } else if algorithm_re.is_match("HuntKillAlgorithm") {
            result = Ok(Algorithm::HuntKillAlgorithm);
        }
        result
    }
}
