import os
from twilio.rest import Client
from dotenv import load_dotenv
import argparse

load_dotenv()

parser = argparse.ArgumentParser()
parser.add_argument("-to", help="Mobile number to make the call to")
parser.add_argument("-m", help="Message to be delivered")
args = parser.parse_args()


account_sid = os.environ["TWILIO_ACCOUNT_SID"]
auth_token = os.environ["TWILIO_AUTH_TOKEN"]

client = Client(account_sid, auth_token)

print("[+] Calling User")

call = client.calls.create(
    to=args.to,
    from_="+16785828539",
    twiml="<Response><Say>stock for " + args.m + " is available</Say></Response>"
)
print("[+] Calling........   SID "+call.sid)
