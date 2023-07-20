import argparse
import aiohttp
import asyncio
import json

HOST = "social.nullrequest.com"

parser = argparse.ArgumentParser()
parser.add_argument("noteid")

args = parser.parse_args()


async def posts(note_id):
    out = ""
    payload = {"noteId": note_id, "limit": 100, "depth": 100}
    async with aiohttp.ClientSession() as session:
        async with session.post(
            f"https://{HOST}/api/notes/children", json=payload
        ) as req:
            replies = await req.json()
            for reply in replies:
                print(json.dumps(reply, indent=4, sort_keys=True))


asyncio.new_event_loop().run_until_complete(posts(args.noteid))
