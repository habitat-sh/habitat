import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/v1');
const globalAny:any = global;

describe('Origin API', function() { 
  describe('Create origin neurosis', function() {
    it('returns the created origin', function(done) {
      request.post('/depot/origins')
        .set('Authorization', globalAny.bobo_bearer)
        .send({"name": "neurosis"})
        .expect(201)
        .end(function(err, res) {
          expect(res.body.name).to.equal("neurosis");
          globalAny.origin_neurosis = res.body;
          done(err);
        });
    });
  });

  describe('Get origin neurosis', function() {
    it('returns the origin', function(done) {
      request.get('/depot/origins/neurosis')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body).to.deep.equal(globalAny.origin_neurosis);
          done(err);
        });
    });
  });

  describe('Create origin xmen', function() {
    it('returns the created origin', function(done) {
      request.post('/depot/origins')
        .set('Authorization', globalAny.logan_bearer)
        .send({"name": "xmen"})
        .expect(201)
        .end(function(err, res) {
          expect(res.body.name).to.equal("xmen");
          globalAny.origin_xmen = res.body;
          done(err);
        });
    });
  });
});
