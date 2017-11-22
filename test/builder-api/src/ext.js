const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

// These magic values correspond to the testpp repo in the habitat-sh org
const installationId = 56940;
const repoId = 114932712;

describe('External API', function() {
  describe('Searching github code', function() {
    it('requires authentication', function(done) {
      request.get(`/ext/installations/${installationId}/search/code`)
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    // This one requires correct GH creds to test. Authenticating with legit GH
    // creds would break the janky session short-circuiting we have for these
    // tests. A bit of a catch-22.
    it('succeeds');
  });

  describe('Getting the external repo content', function() {
    it('requires authentication', function(done) {
      request.get(`/ext/installations/${installationId}/repos/${repoId}/contents/haha`)
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    // This one requires correct GH creds to test. Authenticating with legit GH
    // creds would break the janky session short-circuiting we have for these
    // tests. A bit of a catch-22.
    it('succeeds');
  });

  describe('Validate credentials in an external registry', function() {
    it('requires authentication', function(done) {
      request.post('/ext/integrations/docker/credentials/validate')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    // This one requires correct docker creds to test and I'm not sure how to
    // include those in these tests w/o leaking them to everyone.
    it('succeeds');
  });
});
