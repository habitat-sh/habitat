const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

describe('Origin API', function() {
  describe('Create neurosis origin', function() {
    it('requires authentication', function(done) {
      request.post('/depot/origins')
        .send({'name': 'neurosis'})
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('returns the created origin', function(done) {
      request.post('/depot/origins')
        .set('Authorization', global.boboBearer)
        .send({'name': 'neurosis', 'default_package_visibility': 'private'})
        .expect(201)
        .end(function(err, res) {
          expect(res.body.name).to.equal('neurosis');
          expect(res.body.default_package_visibility).to.equal('private');
          global.originNeurosis = res.body;
          done(err);
        });
    });
  });

  describe('Get origin neurosis', function() {
    it('returns the origin', function(done) {
      request.get('/depot/origins/neurosis')
        .expect(200)
        .end(function(err, res) {
          expect(res.body).to.deep.equal(global.originNeurosis);
          done(err);
        });
    });
  });

  describe('Create xmen origin', function() {
    it('returns the created origin', function(done) {
      request.post('/depot/origins')
        .set('Authorization', global.mystiqueBearer)
        .send({'name': 'xmen'})
        .expect(201)
        .end(function(err, res) {
          expect(res.body.name).to.equal('xmen');
          global.originXmen = res.body;
          done(err);
        });
    });
  });

  describe('Updating origins', function() {
    it('requires authentication', function(done) {
      request.put('/depot/origins/neurosis')
        .send({'default_package_visibility': 'public'})
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires that you are a member of the origin being updated', function(done) {
      request.put('/depot/origins/neurosis')
        .set('Authorization', global.mystiqueBearer)
        .send({'default_package_visibility': 'public'})
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.put('/depot/origins/neurosis')
        .set('Authorization', global.boboBearer)
        .send({'default_package_visibility': 'public'})
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('reflects the changes when viewing it again', function(done) {
      request.get('/depot/origins/neurosis')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.default_package_visibility).to.equal('public');
          global.originNeurosis = res.body;
          done(err);
        });
    });
  });
});
