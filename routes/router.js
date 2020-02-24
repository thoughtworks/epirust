var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.sendFile('../public/index.html');
});

router.use('/simulation', require('../controllers/simulation'));

module.exports = router;