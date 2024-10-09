Used https://github.com/vgrichina/web4 and web3.storage
Owner is crans.near, modify the contract if you want to have the access.

> Contract Structure:

    /owner_id: The account that can modify the contract.
    /base_url: The base URL for all pages.
    /pages: A map of path strings to file names.


> Functions:
  a. new(owner_id: AccountId, base_url: String):

    /Initializes the contract.
    /Sets the owner and base URL.
    /Adds a default index page.

  b. add_page(path: String, file_name: String):

    /Adds a new page to the contract.
    /Only the owner can call this.

  c. get_page_url(path: String) -> Option<String>:

    /Retrieves the full URL for a given path.

  d. set_base_url(new_base_url: String):

    /Changes the base URL.
    /Only the owner can call this.

  e. get_base_url() -> String:

    /Retrieves the current base URL.

  f. web4_get(request: Web4Request) -> Web4Response:

    /Handles Web4 requests.
    /Returns appropriate responses based on the requested path.
