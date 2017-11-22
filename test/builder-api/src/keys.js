const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');
const fs = require('fs');

const revision = '20171211220037';
const pubFile = fs.readFileSync(__dirname + `/../fixtures/neurosis-${revision}.pub`, 'utf8');
const secretFile = fs.readFileSync(__dirname + `/../fixtures/neurosis-${revision}.sig.key`, 'utf8');

describe('Keys API', function() {
  describe('Uploading public keys', function() {
    it('requires authentication', function(done) {
      request.post(`/depot/origins/neurosis/keys/${revision}`)
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin you are uploading to', function(done) {
      request.post(`/depot/origins/neurosis/keys/${revision}`)
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('uploads the key', function(done) {
      request.post(`/depot/origins/neurosis/keys/${revision}`)
        .set('Authorization', global.boboBearer)
        .send(pubFile)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.equal(`/origins/neurosis/keys/${revision}`);
          // JB TODO: this is wrong - this URL doesn't go anywhere in our
          // system
          expect(res.header['location']).to.equal(`http://localhost:9636/key/neurosis-${revision}`);
          done(err);
        });
    });
  });

  describe('Downloading public keys', function() {
    it('can download a specific revision', function(done) {
      request.get(`/depot/origins/neurosis/keys/${revision}`)
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.equal(pubFile);
          done(err);
        });
    });

    it('can download the latest key', function(done) {
      request.get('/depot/origins/neurosis/keys/latest')
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.equal(pubFile);
          done(err);
        });
    });
  });

  describe('Uploading secret keys', function() {
    it('requires authentication', function(done) {
      request.post(`/depot/origins/neurosis/secret_keys/${revision}`)
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin you are uploading to', function(done) {
      request.post(`/depot/origins/neurosis/secret_keys/${revision}`)
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('uploads the key', function(done) {
      request.post(`/depot/origins/neurosis/secret_keys/${revision}`)
        .set('Authorization', global.boboBearer)
        .send(secretFile)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Downloading secret keys', function() {
    it('requires authentication', function(done) {
      request.get('/depot/origins/neurosis/secret_keys/latest')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin you are uploading to', function(done) {
      request.get('/depot/origins/neurosis/secret_keys/latest')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('can download the latest key', function(done) {
      request.get('/depot/origins/neurosis/secret_keys/latest')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.equal(secretFile);
          done(err);
        });
    });
  });

  describe('Generating keys', function() {
    it('requires authentication', function(done) {
      request.post('/depot/origins/neurosis/keys')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin you are uploading to', function(done) {
      request.post('/depot/origins/neurosis/keys')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('generates the key', function(done) {
      request.post('/depot/origins/neurosis/keys')
        .set('Authorization', global.boboBearer)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Listing keys', function() {
    it('can list all public keys', function(done) {
      request.get('/depot/origins/neurosis/keys')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.length).to.equal(2);
          expect(res.body[0].origin).to.equal('neurosis');
          expect(parseInt(res.body[0].revision) > parseInt(res.body[1].revision)).to.be.true;
          expect(res.body[0].location).to.equal(`/origins/neurosis/keys/${res.body[0].revision}`);
          expect(res.body[1].origin).to.equal('neurosis');
          expect(res.body[1].revision).to.equal(revision);
          expect(res.body[1].location).to.equal(`/origins/neurosis/keys/${revision}`);
          done(err);
        });
    });
  });

  describe('Builder keys', function() {
    it('can retrieve the latest key', function(done) {
      request.get('/depot/builder/keys/latest')
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.include('BOX-PUB-1');
          expect(res.text.match(/bldr-[0-9]{14}/)).to.not.be.null;
          done(err);
        });
    });
  });
});
