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
    return [
        {
            "id": img["id"],
            "title": f"Image {index + 1}",
            "url": img["largeImageURL"]
        }
        for index, img in enumerate(search_results["hits"])
    ]

parser = argparse.ArgumentParser(description="Retrieve image links using the Pixabay API")
parser.add_argument("--api_key", required=True, help="Pixabay API key")
parser.add_argument("--search_term", required=True, help="Search term for images")
parser.add_argument("--count", type=int, default=10, help="Number of images to retrieve (default: 10)")
args = parser.parse_args()

images_info = search_images_pixabay(args.api_key, args.search_term, args.count)

with open('image_data.json', 'w') as f:
    json.dump(images_info, f, indent=4)
