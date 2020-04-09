export default {
  API_HOST: process.env.REACT_APP_API_HOST === undefined ? 'http://localhost:3000' : process.env.REACT_APP_API_HOST
}