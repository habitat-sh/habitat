const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

describe('Channels API', function() {
  describe('Create foo channel', function() {
    it('requires authentication to create a channel', function(done) {
      request.post('/depot/channels/neurosis/foo')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('returns the created channel', function(done) {
      request.post('/depot/channels/neurosis/foo')
        .set('Authorization', global.boboBearer)
        .expect(201)
        .end(function(err, res) {
          expect(res.body.name).to.equal('foo');
          expect(res.body.owner_id).to.equal(global.sessionBobo.id);
          global.channelFoo = res.body;
          done(err);
        });
    });
  });

  describe('Create bar channel', function() {
    it('succeeds', function(done) {
      request.post('/depot/channels/neurosis/bar')
        .set('Authorization', global.boboBearer)
        .expect(201)
        .end(function(err, res) {
          expect(res.body.name).to.equal('bar');
          expect(res.body.owner_id).to.equal(global.sessionBobo.id);
          global.channelBar = res.body;
          done(err);
        });
    });
  });

  describe('Channel promotion', function() {
    it('requires authentication to promote a package', function(done) {
      request.put('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213/promote')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires origin membership to promote a package', function(done) {
      request.put('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213/promote')
        .set('Authorization', global.mystiqueBearer)
        // JB TODO: existing behavior is incorrect - this should return a 403 not a 401
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });

    it('puts the specified package into the specified channel', function(done) {
      request.put('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213/promote')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('can promote private packages', function(done) {
      request.put('/depot/channels/neurosis/bar/pkgs/testapp/0.1.3/20171206004121/promote')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Listing packages in a channel', function() {
    it('returns all packages in a channel', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(0);
          expect(res.body.total_count).to.equal(1);
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.3');
          expect(res.body.data[0].release).to.equal('20171205003213');
          done(err);
        });
    });

    it('returns all packages with the given name in a channel', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(0);
          expect(res.body.total_count).to.equal(1);
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.3');
          expect(res.body.data[0].release).to.equal('20171205003213');
          done(err);
        });
    });

    it('returns all packages with the given name and version in a channel', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(0);
          expect(res.body.total_count).to.equal(1);
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.3');
          expect(res.body.data[0].release).to.equal('20171205003213');
          done(err);
        });
    });

    it('returns the package with the given name, version and release in a channel', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal('20171205003213');
          done(err);
        });
    });

    it('returns the latest package with the given name in a channel', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs/testapp/latest')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal('20171205003213');
          done(err);
        });
    });

    it('returns the latest package with the given name and version in a channel', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/latest')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal('20171205003213');
          done(err);
        });
    });

    it('requires authentication to view private packages in a channel', function(done) {
      request.get('/depot/channels/neurosis/bar/pkgs/testapp/0.1.3/latest')
        .type('application/json')
        .accept('application/json')
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('does not let members of other origins view private packages in a channel', function(done) {
      request.get('/depot/channels/neurosis/bar/pkgs/testapp/0.1.3/latest')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('allows members of the origin to view private packages when they are authenticated', function(done) {
      request.get('/depot/channels/neurosis/bar/pkgs/testapp/0.1.3/latest')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal('20171206004121');
          done(err);
        });
    });
  });

  describe('Listing channels in an origin', function() {
    it('returns a list of channels', function(done) {
      request.get('/depot/channels/neurosis')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.length).to.equal(4);
          expect(res.body[0].name).to.equal('bar');
          expect(res.body[1].name).to.equal('foo');
          expect(res.body[2].name).to.equal('stable');
          expect(res.body[3].name).to.equal('unstable');
          done(err);
        });
    });
  });

  describe('Channel demotion', function() {
    it('requires authentication to demote a package', function(done) {
      request.put('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213/demote')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires origin membership to demote a package', function(done) {
      request.put('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213/demote')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          done(err);
        });
    });

    it('removes the specified package from the specified channel', function(done) {
      request.put('/depot/channels/neurosis/foo/pkgs/testapp/0.1.3/20171205003213/demote')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('rejects attempts to demote a package from the unstable channel', function(done) {
      request.put('/depot/channels/neurosis/unstable/pkgs/testapp/0.1.3/20171205003213/demote')
        .set('Authorization', global.boboBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('will not find a package in a channel after it has been demoted', function(done) {
      request.get('/depot/channels/neurosis/foo/pkgs')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(49);
          expect(res.body.total_count).to.equal(0);
          expect(res.body.data.length).to.equal(0);
          done(err);
        });
    });
  });

  describe('Delete foo channel', function() {
    it('rejects attempts to delete the stable channel', function(done) {
      request.delete('/depot/channels/neurosis/stable')
        .set('Authorization', global.boboBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('rejects attempts to delete the unstable channel', function(done) {
      request.delete('/depot/channels/neurosis/unstable')
        .set('Authorization', global.boboBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires authentication to delete a channel', function(done) {
      request.delete('/depot/channels/neurosis/foo')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires origin membership to delete a channel', function(done) {
      request.delete('/depot/channels/neurosis/foo')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.delete('/depot/channels/neurosis/foo')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });
});
