import argparse
import requests
import json

def search_images_pixabay(api_key, search_term, count):
    search_url = "https://pixabay.com/api/"
    params = {
        "key": api_key,
        "q": search_term,
        "per_page": count
    }
    response = requests.get(search_url, params=params)
    response.raise_for_status()
    search_results = response.json()
    return [img["largeImageURL"] for img in search_results["hits"]]

# Parse command-line arguments
parser = argparse.ArgumentParser(description="Retrieve image links using the Pixabay API")
parser.add_argument("--api_key", required=True, help="Pixabay API key")
parser.add_argument("--search_term", required=True, help="Search term for images")
parser.add_argument("--count", type=int, default=20, help="Number of images to retrieve (default: 10)")
args = parser.parse_args()

# Retrieve image links
image_urls = search_images_pixabay(args.api_key, args.search_term, args.count)

# Save image links to a JSON file
with open('image_urls.json', 'w') as f:
    json.dump(image_urls, f)
