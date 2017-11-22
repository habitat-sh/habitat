const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

describe('Integrations API', function() {
  describe('Creating an origin integration', function() {
    it('requires authentication', function(done) {
      request.put('/depot/origins/neurosis/integrations/docker/foo')
        .type('application/json')
        .accept('application/json')
        .send({
          some: 'data',
          random: true,
          does_not_matter: 'haha'
        })
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the given origin', function(done) {
      request.put('/depot/origins/neurosis/integrations/docker/foo')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .send({
          some: 'data',
          random: true,
          does_not_matter: 'haha'
        })
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.put('/depot/origins/neurosis/integrations/docker/foo')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send({
          some: 'data',
          random: true,
          does_not_matter: 'haha'
        })
        // JB TODO: this should be 201, not 204
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Retrieving all integrations for an origin', function() {
    it('requires authentication', function(done) {
      request.get('/depot/origins/neurosis/integrations')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the given origin', function(done) {
      request.get('/depot/origins/neurosis/integrations')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.get('/depot/origins/neurosis/integrations')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.docker).to.deep.equal(['foo']);
          done(err);
        });
    });
  });

  describe('Retrieving all names for a specific integration', function() {
    it('requires authentication', function(done) {
      request.get('/depot/origins/neurosis/integrations/docker/names')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the given origin', function(done) {
      request.get('/depot/origins/neurosis/integrations/docker/names')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.get('/depot/origins/neurosis/integrations/docker/names')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.names).to.deep.equal(['foo']);
          done(err);
        });
    });
  });

  describe('Removing an integration', function() {
    it('requires authentication', function(done) {
      request.delete('/depot/origins/neurosis/integrations/docker/foo')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the given origin', function(done) {
      request.delete('/depot/origins/neurosis/integrations/docker/foo')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.delete('/depot/origins/neurosis/integrations/docker/foo')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('creates another integration so that future tests will pass', function(done) {
      request.put('/depot/origins/neurosis/integrations/docker/docker')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send({
          some: 'data',
          random: true,
          does_not_matter: 'haha'
        })
        // JB TODO: this should be 201, not 204
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    })
  });
});
