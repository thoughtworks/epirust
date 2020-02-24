/* GET simulation listing. */
const app = require('express');
const router = app.Router();

router.get('/', function(req, res, next) {
  res.send("In simulation");
});

module.exports = router;
