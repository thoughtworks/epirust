import config from "../config";

export const post = (url, data) => {
  return fetch(`${config.API_HOST}/api${url}`, {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify(data)
  })
    .then(response => handleNon200(response))
}

export const get = (url) => {
  return fetch(`${config.API_HOST}/api${url}`, {
    method: 'GET',
    headers: {'Content-Type': 'application/json'}
  })
    .then(response => handleNon200(response))
}

const handleNon200 = (response) => {
  if (!response.ok) {
    throw Error(response.statusText);
  }
  return response;
}