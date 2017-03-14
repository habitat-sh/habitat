import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/v1');
const globalAny:any = global;

describe('Authenticate API', function() { 
  describe('Create sessions', function() {
    it('returns bobo', function(done) {
      request.get('/authenticate/bobo')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("bobo");
          globalAny.session_bobo = res.body;
          done(err);
        });
    });
    it('returns logan', function(done) {
      request.get('/authenticate/logan')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("logan");
          globalAny.session_logan = res.body;
          done(err);
        });
    });
  });
});


