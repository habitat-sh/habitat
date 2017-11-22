const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

describe('Authenticate API', function() {
  describe('Create sessions', function() {
    it('returns bobo', function(done) {
      request.get('/authenticate/bobo')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal('bobo');
          global.sessionBobo = res.body;
          done(err);
        });
    });

    it('returns mystique', function(done) {
      request.get('/authenticate/mystique')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal('mystique');
          global.sessionMystique = res.body;
          done(err);
        });
    });
  });
});
