mod pagerank_impl;
// Importing the fill function from the textwrap crate to wrap text at 78 characters per line.
use textwrap::fill;

fn main() {
    // The graph represents links between sports websites. Each index represents a website,
    // and the values in the vectors are the indexes of the websites they link to.
    let graph = vec![
        vec![1, 2], // ESPN links to NFL, NBA
        vec![0],    // NFL links to ESPN
        vec![0, 3], // NBA links to ESPN, UFC
        vec![0],    // UFC links to ESPN
        vec![0, 1], // MLB links to ESPN, NFL
    ];

    // The names corresponding to the indexes of the websites.
    let names = ["ESPN", "NFL", "NBA", "UFC", "MLB"];

    let pagerank = pagerank_impl::PageRank::new(0.85, 100);
    let ranks = pagerank.rank(&graph);

    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";
    println!("{}", fill(explanation, 78));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank() {
        let graph = vec![
            vec![1, 2],
            vec![0],
            vec![0, 3],
            vec![0],
            vec![0, 1],
        ];

        let pagerank = pagerank_impl::PageRank::new(0.85, 100);
        let ranks = pagerank.rank(&graph);

        assert_eq!(ranks.len(), 5);
        assert_eq!(ranks.iter().sum::<f64>(), 1.0);
    }
}