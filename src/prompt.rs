use anyhow::bail;
use anyhow::Result;
use regex::Regex;

use std::collections::{HashMap, HashSet};

pub fn format_prompt(prompt: &str, map: HashMap<&str, &str>) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("<([A-Z_]+)>").unwrap();
    }

    let required_keys: HashSet<String> = RE
        .captures_iter(prompt)
        .map(|cap| cap[1].to_string())
        .collect();
    let provided_keys: HashSet<String> = map.keys().map(|s| s.to_string()).collect();

    if !required_keys.eq(&provided_keys) {
        bail!(
            r#"Required keys did not match provided keys.
  Required: {:?}
  Provided: {:?}
  Prompt: {}"#,
            required_keys,
            provided_keys,
            prompt
        );
    }

    let mut result = prompt.to_string();
    for (key, value) in map {
        result = result.replace(&format!("<{}>", key), value);
    }
    Ok(result)
}

pub static PROMPT_TO_SUMMARIZE_DIFF: &str = r#"You are an expert programmer, and you are trying to summarize a git diff.
Reminders about the git diff format:
For every file, there are a few metadata lines, like (for example):
```
diff --git a/lib/index.js b/lib/index.js
index aadf691..bfef603 100644
--- a/lib/index.js
+++ b/lib/index.js
```
This means that `lib/index.js` was modified in this commit. Note that this is only an example.
Then there is a specifier of the lines that were modified.
A line starting with `+` means it was added.
A line that starting with `-` means that line was deleted.
A line that starts with neither `+` nor `-` is code given for context and better understanding.
It is not part of the diff.
After the git diff of the first file, there will be an empty line, and then the git diff of the next file.

Do not include the file name as another part of the comment.
Do not use the characters `[` or `]` in the summary.
Write every summary comment in a new line.
Comments should be in a bullet point list, each line starting with a `-`.
The summary should not include comments copied from the code.
The output should be easily readable. When in doubt, write less comments and not more. Do not output comments that simply repeat the contents of the file.
Readability is top priority. Write only the most important comments about the diff.

EXAMPLE SUMMARY COMMENTS:
```
- Raise the amount of returned recordings from `10` to `100`
- Fix a typo in the github action name
- Move the `octokit` initialization to a separate file
- Add an OpenAI API for completions
- Lower numeric tolerance for test files
- Add 2 tests for the inclusive string split function
```
Most commits will have less comments than this examples list.
The last comment does not include the file names,
because there were more than two relevant files in the hypothetical commit.
Do not include parts of the example in your summary.
It is given only as an example of appropriate comments.


THE GIT DIFF TO BE SUMMARIZED:
```
<FILE_DIFF>
```

THE SUMMARY:

"#;

pub static PROMPT_TO_SUMMARIZE_DIFF_SUMMARIES: &str = r#"You are an expert programmer, and you are trying to summarize a pull request.
You went over every file that was changed in it.
For some of these files changes where too big and were omitted in the files diff summary.
Please summarize the pull request.
Write your response in bullet points, using the imperative tense following the pull request style guide.
Starting each bullet point with a `-`.
Write a high level description. Do not repeat the commit summaries or the file summaries.
Write the most important bullet points. The list should not be more than a few bullet points.

THE FILE SUMMARIES:
```
<SUMMARY_POINTS>
```

Remember to write only the most important points and do not write more than a few bullet points.
THE PULL REQUEST SUMMARY:

"#;

pub static PROMPT_TO_SUMMARIZE_DIFF_TITLE: &str = r#"You are an expert programmer, and you are trying to title a pull request.
You went over every file that was changed in it.
For some of these files changes where too big and were omitted in the files diff summary.
Please summarize the pull request into a single specific theme.
Write your response using the imperative tense following the kernel git commit style guide.
Write a high level title.
Do not repeat the commit summaries or the file summaries.
Do not list individual changes in the title.

EXAMPLE SUMMARY COMMENTS:
```
Raise the amount of returned recordings
Switch to internal API for completions
Lower numeric tolerance for test files
Schedule all GitHub actions on all OSs
```

THE FILE SUMMARIES:
```
<SUMMARY_POINTS>
```

Remember to write only one line, no more than 50 characters.
THE PULL REQUEST TITLE:

"#;
