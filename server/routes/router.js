var express = require('express');
var router = express.Router();
var simulation = require('../controllers/simulation');

/* GET home page. */
router.get('/', function(req, res, next) {
  res.sendFile('../public/index.html');
});

router.use('/simulation', simulation);

module.exports = router;