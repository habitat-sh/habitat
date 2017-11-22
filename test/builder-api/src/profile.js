const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

describe('Profile API', function() {
  describe('Updating the profile', function() {
    it('requires authentication', function(done) {
      request.patch('/profile')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('allows someone to update their profile', function(done) {
      request.patch('/profile')
        .set('Authorization', global.boboBearer)
        .type('application/json')
        .accept('application/json')
        .send({email: 'haha@example.com'})
        .expect(200)
        .end(function(err, res) {
          // JB TODO: this is a sub-optimal UX here. we should return the
          // updated profile.
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Retrieving a profile', function() {
    it('requires authentication', function(done) {
      request.get('/profile')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('allows someone to retrieve their profile', function(done) {
      request.get('/profile')
        .set('Authorization', global.boboBearer)
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.email).to.equal('haha@example.com');
          done(err);
        });
    });
  });
});
