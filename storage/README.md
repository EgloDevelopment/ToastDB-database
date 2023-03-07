# ToastDB-database

This is a database designed for Eglo use, mostly for storing user messages, or logs, or just non mission-critical information.

This databse **IS NOT** designed for speed or efficency, but rather for scalability and reliability and simplicity of use.

# Scalability

ToastDB is designed for easy scalability, just create a Docker app and navigate to the dashboard and add the URL, ToastDB will handle the rest.

# Ease of access

ToastDB is accessible over HTTP POST requests, and a custom query language we call TDQL (ToastDbQueryLanguage)

(Over a POST request to a URL like: https://your-server/api/query)

The main reason for performing queries like this is that no custom drivers need to be made, ust use *cURL* or *Axios*

# Caveats

For speed and high access, we would recommend MongoDB, or PostgreSQL.

For security, run all of the "storage" nodes behind Tailscale or a VPN, and then expose the main server to the internet.
