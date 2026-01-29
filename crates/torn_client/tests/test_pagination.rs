//! Integration tests for pagination behavior.
//!
//! Tests that pagination navigation works correctly with the real API.
//! All tests skip gracefully when TORN_API_KEY is not set.

mod common;

#[tokio::test]
async fn pagination_has_next_detection() {
    let Some(client) = common::test_client() else {
        common::skip_message("pagination_has_next_detection");
        return;
    };

    let result = client.user().attacks().await;
    assert!(result.is_ok(), "user.attacks() failed: {:?}", result.err());

    let response = result.unwrap();
    // Check if has_next works (may be true or false depending on data)
    let _ = response.has_next();
    let _ = response.has_next_page();
}

#[tokio::test]
async fn pagination_next_url_extraction() {
    let Some(client) = common::test_client() else {
        common::skip_message("pagination_next_url_extraction");
        return;
    };

    let result = client.user().attacks().await;
    assert!(result.is_ok(), "user.attacks() failed: {:?}", result.err());

    let response = result.unwrap();

    // If there's a next URL, it should be properly formatted
    if let Some(next_url) = response.next_url() {
        assert!(next_url.contains("api.torn.com"), "next_url should be a valid Torn API URL");
    }
}

#[tokio::test]
async fn pagination_navigation() {
    let Some(client) = common::test_client() else {
        common::skip_message("pagination_navigation");
        return;
    };

    let result = client.user().attacks().await;
    assert!(result.is_ok(), "user.attacks() failed: {:?}", result.err());

    let response = result.unwrap();

    // If there's a next page, try to fetch it
    if response.has_next() {
        let next_result = response.next().await;
        assert!(next_result.is_ok(), "pagination.next() failed: {:?}", next_result.err());

        let next_page = next_result.unwrap();
        assert!(next_page.is_some(), "pagination.next() returned None when has_next was true");

        if let Some(page) = next_page {
            // Verify the next page has data
            assert!(page.data.attacks.is_some(), "next page missing attacks data");
        }
    }
}

#[tokio::test]
async fn pagination_stream() {
    let Some(client) = common::test_client() else {
        common::skip_message("pagination_stream");
        return;
    };

    let result = client.user().attacks().await;
    assert!(result.is_ok(), "user.attacks() failed: {:?}", result.err());

    let response = result.unwrap();

    // Use the page stream to iterate (limit to 3 pages max to avoid long test)
    let mut pages = response.pages();
    let mut page_count = 0;

    while let Some(page_result) = pages.next_page().await {
        assert!(page_result.is_ok(), "page stream failed: {:?}", page_result.err());
        page_count += 1;

        // Limit iterations to avoid long-running test
        if page_count >= 3 {
            break;
        }
    }

    assert!(page_count > 0, "page stream yielded no pages");
}

#[tokio::test]
async fn pagination_faction_members() {
    let Some(client) = common::test_client() else {
        common::skip_message("pagination_faction_members");
        return;
    };

    // Test pagination with faction members (using a known large faction)
    let result = client.faction().with_id(10000).members().await;

    // May fail if API key doesn't have access
    if result.is_err() {
        eprintln!("faction.members() failed (expected if no access) - skipping pagination test");
        return;
    }

    let response = result.unwrap();
    // Just verify the pagination structure exists
    let _ = response.has_next();
}
