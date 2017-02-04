extern crate url;

use std::process::Command;

use url::Url;

fn main() {
    let url = Url::parse("https://gitlab.\
                          com/api/v3/projects/2208459/repository/files?file_path=message.\
                          proto&ref=develop");

    let private_token =
        Command::new("cat").arg("$GITLAB_KEY").output().expect("Failed to get private token");
}
