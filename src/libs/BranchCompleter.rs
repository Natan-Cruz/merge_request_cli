
use inquire::{
    CustomUserError, 
    Autocomplete, 
    autocompletion::Replacement
};

use crate::{GitFunctions, Utils};

#[derive(Clone, Default)]
pub struct BranchCompleter {
    input: String,
    paths: Vec<String>,
    lcp: String,
    branchs: Vec<String>
}

impl BranchCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }

        let all_branchs = GitFunctions::get_all_branchs();
        let mut branchs_splitted:Vec<String> = Vec::new();

        if self.branchs.is_empty() {
            let branchs_splitted: Vec<String> = Utils::transform_string_to_vec(all_branchs); 
            self.branchs = branchs_splitted 
        } else {
            branchs_splitted = self.branchs.clone();
        };

        let entries: Vec<String> = branchs_splitted
            .into_iter()
            .filter(|s| s.starts_with(input))
            .collect();

        self.paths = entries;
        self.lcp = self.longest_common_prefix();

        Ok(())
    }

    fn longest_common_prefix(&self) -> String {
        let mut ret: String = String::new();

        let mut sorted = self.paths.clone();
        sorted.sort();
        if sorted.is_empty() {
            return ret
        }

        let mut first_word = sorted.first().unwrap().chars();
        let mut last_word = sorted.last().unwrap().chars();

        loop {
            match (first_word.next(), last_word.next()) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    ret.push(c1);
                }
                _ => return ret,
            }
        }
    }
}

impl Autocomplete for BranchCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.paths.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => match self.lcp.is_empty() {
                true => Replacement::None,
                false => Replacement::Some(self.lcp.clone())
            }
        })

    }
}
