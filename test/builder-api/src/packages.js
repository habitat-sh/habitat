const expect = require('chai').expect;
const supertest = require('supertest');
const binaryParser = require('superagent-binary-parser');
const request = supertest('http://localhost:9636/v1');
const fs = require('fs');

const release1 = '20171205003213';
const release2 = '20171206004121';
const release3 = '20171206004139';
const release4 = '20171206005217';

const file1 = fs.readFileSync(__dirname + `/../fixtures/neurosis-testapp-0.1.3-${release1}-x86_64-linux.hart`);
const file2 = fs.readFileSync(__dirname + `/../fixtures/neurosis-testapp-0.1.3-${release2}-x86_64-linux.hart`);
const file3 = fs.readFileSync(__dirname + `/../fixtures/neurosis-testapp-0.1.4-${release3}-x86_64-linux.hart`);
const file4 = fs.readFileSync(__dirname + `/../fixtures/xmen-testapp-0.1.4-${release4}-x86_64-linux.hart`);
var downloadedPath = '/tmp/';

describe('Working with packages', function() {
  describe('Uploading packages', function() {
    it('does not allow unauthenticated users to upload packages', function(done) {
      request.post(`/depot/pkgs/neurosis/testapp/0.1.3/${release1}`)
        .query({checksum: '3138777020e7bb621a510b19c2f2630deee9b34ac11f1c2a0524a44eb977e4a8'})
        .set('Content-Length', file1.length)
        .send(file1)
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires that you are a member of the origin to upload a package', function(done) {
      request.post(`/depot/pkgs/neurosis/testapp/0.1.3/${release1}`)
        .set('Authorization', global.mystiqueBearer)
        .set('Content-Length', file1.length)
        .query({checksum: '3138777020e7bb621a510b19c2f2630deee9b34ac11f1c2a0524a44eb977e4a8'})
        .send(file1)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('allows authenticated users to upload packages', function(done) {
      request.post(`/depot/pkgs/neurosis/testapp/0.1.3/${release1}`)
        .set('Authorization', global.boboBearer)
        .set('Content-Length', file1.length)
        .query({checksum: '3138777020e7bb621a510b19c2f2630deee9b34ac11f1c2a0524a44eb977e4a8'})
        .send(file1)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.equal(`/pkgs/neurosis/testapp/0.1.3/${release1}/download`);
          done(err);
        });
    });

    it('uploads a second package', function(done) {
      request.post(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}`)
        .set('Authorization', global.boboBearer)
        .set('Content-Length', file2.length)
        .query({checksum: 'd8943c86636eb0a24cb63a80b3d9375ce342f2fa192375f3a0b83eab44de21eb'})
        .send(file2)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.equal(`/pkgs/neurosis/testapp/0.1.3/${release2}/download`);
          done(err);
        });
    });

    it('uploads a third package', function(done) {
      request.post(`/depot/pkgs/neurosis/testapp/0.1.4/${release3}`)
        .set('Authorization', global.boboBearer)
        .set('Content-Length', file3.length)
        .query({checksum: '1fa27a110fe01acba9d31a0f56801c5e38f4feb8105266231f308091e487c6d1'})
        .send(file3)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.equal(`/pkgs/neurosis/testapp/0.1.4/${release3}/download`);
          done(err);
        });
    });

    it('uploads a fourth package', function(done) {
      request.post(`/depot/pkgs/xmen/testapp/0.1.4/${release4}`)
        .set('Authorization', global.mystiqueBearer)
        .set('Content-Length', file4.length)
        .query({checksum: 'b1661779dd7dcef994ae0ab4c2c3c589dde56747d91511cb44a41813831336a1'})
        .send(file4)
        .expect(201)
        .end(function(err, res) {
          expect(res.text).to.equal(`/pkgs/xmen/testapp/0.1.4/${release4}/download`);
          done(err);
        });
    });
  });

  describe('Finding packages', function() {
    it('allows me to search for packages', function(done) {
      request.get('/depot/pkgs/search/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(3);
          expect(res.body.total_count).to.equal(4);
          expect(res.body.data.length).to.equal(4);
          expect(res.body.data[0].origin).to.equal('neurosis');
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.3');
          expect(res.body.data[0].release).to.equal(release1);
          expect(res.body.data[1].origin).to.equal('neurosis');
          expect(res.body.data[1].name).to.equal('testapp');
          expect(res.body.data[1].version).to.equal('0.1.3');
          expect(res.body.data[1].release).to.equal(release2);
          expect(res.body.data[2].origin).to.equal('neurosis');
          expect(res.body.data[2].name).to.equal('testapp');
          expect(res.body.data[2].version).to.equal('0.1.4');
          expect(res.body.data[2].release).to.equal(release3);
          expect(res.body.data[3].origin).to.equal('xmen');
          expect(res.body.data[3].name).to.equal('testapp');
          expect(res.body.data[3].version).to.equal('0.1.4');
          expect(res.body.data[3].release).to.equal(release4);
          done(err);
        });
    });

    it('lists all packages', function(done) {
      request.get('/depot/pkgs/neurosis')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(2);
          expect(res.body.total_count).to.equal(3);
          expect(res.body.data.length).to.equal(3);
          expect(res.body.data[0].origin).to.equal('neurosis');
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.4');
          expect(res.body.data[0].release).to.equal(release3);
          expect(res.body.data[1].origin).to.equal('neurosis');
          expect(res.body.data[1].name).to.equal('testapp');
          expect(res.body.data[1].version).to.equal('0.1.3');
          expect(res.body.data[1].release).to.equal(release2);
          expect(res.body.data[2].origin).to.equal('neurosis');
          expect(res.body.data[2].name).to.equal('testapp');
          expect(res.body.data[2].version).to.equal('0.1.3');
          expect(res.body.data[2].release).to.equal(release1);
          done(err);
        });
    });

    it('lists all unique package names', function(done) {
      request.get('/depot/neurosis/pkgs')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(0);
          expect(res.body.total_count).to.equal(1);
          expect(res.body.data.length).to.equal(1);
          expect(res.body.data[0].origin).to.equal('neurosis');
          expect(res.body.data[0].name).to.equal('testapp');
          done(err);
        });
    });

    it('lists all packages with the specified name', function(done) {
      request.get('/depot/pkgs/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(2);
          expect(res.body.total_count).to.equal(3);
          expect(res.body.data.length).to.equal(3);
          expect(res.body.data[0].origin).to.equal('neurosis');
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.4');
          expect(res.body.data[0].release).to.equal(release3);
          expect(res.body.data[1].origin).to.equal('neurosis');
          expect(res.body.data[1].name).to.equal('testapp');
          expect(res.body.data[1].version).to.equal('0.1.3');
          expect(res.body.data[1].release).to.equal(release2);
          expect(res.body.data[2].origin).to.equal('neurosis');
          expect(res.body.data[2].name).to.equal('testapp');
          expect(res.body.data[2].version).to.equal('0.1.3');
          expect(res.body.data[2].release).to.equal(release1);
          done(err);
        });
    });

    it('lists all versions of the package with the specified name', function(done) {
      request.get('/depot/pkgs/neurosis/testapp/versions')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.length).to.equal(2);
          expect(res.body[0].origin).to.equal('neurosis');
          expect(res.body[0].name).to.equal('testapp');
          expect(res.body[0].version).to.equal('0.1.4');
          expect(res.body[0].release_count).to.equal('1');
          expect(res.body[0].latest).to.equal(release3);
          expect(res.body[0].platforms.length).to.equal(1);
          expect(res.body[0].platforms[0]).to.equal('x86_64-linux');
          expect(res.body[1].origin).to.equal('neurosis');
          expect(res.body[1].name).to.equal('testapp');
          expect(res.body[1].version).to.equal('0.1.3');
          expect(res.body[1].release_count).to.equal('2');
          expect(res.body[1].latest).to.equal(release2);
          expect(res.body[1].platforms.length).to.equal(1);
          expect(res.body[1].platforms[0]).to.equal('x86_64-linux');
          done(err);
        });
    });

    it('returns the latest release of a package with the specified name', function(done) {
      request.get('/depot/pkgs/neurosis/testapp/latest')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.4');
          expect(res.body.ident.release).to.equal(release3);
          done(err);
        });
    });

    it('lists all packages with the specified name and version', function(done) {
      request.get('/depot/pkgs/neurosis/testapp/0.1.3')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(1);
          expect(res.body.total_count).to.equal(2);
          expect(res.body.data.length).to.equal(2);
          expect(res.body.data[0].origin).to.equal('neurosis');
          expect(res.body.data[0].name).to.equal('testapp');
          expect(res.body.data[0].version).to.equal('0.1.3');
          expect(res.body.data[0].release).to.equal(release2);
          expect(res.body.data[1].origin).to.equal('neurosis');
          expect(res.body.data[1].name).to.equal('testapp');
          expect(res.body.data[1].version).to.equal('0.1.3');
          expect(res.body.data[1].release).to.equal(release1);
          done(err);
        });
    });

    it('returns the latest release of a package with the spcified name and version', function(done) {
      request.get('/depot/pkgs/neurosis/testapp/0.1.3/latest')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal(release2);
          done(err);
        });
    });

    it('returns the specified release', function(done) {
      request.get(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}`)
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal(release2);
          done(err);
        });
    });
  });

  describe('Other functions', function() {
    it('lists all the channels a package is in', function(done) {
      request.get(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}/channels`)
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.length).to.equal(1);
          expect(res.body[0]).to.equal('unstable');
          done(err);
        });
    });

    it('downloads a package', function(done) {
      request.get(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}/download`)
        .expect(200)
        .buffer()
        .parse(binaryParser)
        .end(function(err, res) {
          var name = res.header['x-filename'];
          var path = downloadedPath + name;
          fs.writeFileSync(path, res.body);
          var size = fs.statSync(path).size;
          expect(name).to.equal(`neurosis-testapp-0.1.3-${release2}-x86_64-linux.hart`)
          expect(size).to.equal(1569);
          done(err);
        });
    });

    it('toggles the privacy setting for a package', function(done) {
      request.patch(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}/private`)
        .set('Authorization', global.boboBearer)
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires authentication to view private packages', function(done) {
      request.get(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}`)
        .type('application/json')
        .accept('application/json')
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('does not let members of other origins view private packages', function(done) {
      request.get(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}`)
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('allows members of the origin to view private packages when they are authenticated', function(done) {
      request.get(`/depot/pkgs/neurosis/testapp/0.1.3/${release2}`)
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.ident.origin).to.equal('neurosis');
          expect(res.body.ident.name).to.equal('testapp');
          expect(res.body.ident.version).to.equal('0.1.3');
          expect(res.body.ident.release).to.equal(release2);
          done(err);
        });
    });

    it('returns package stats', function(done) {
      request.get('/depot/pkgs/origins/neurosis/stats')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body.plans).to.equal(3);
          expect(res.body.builds).to.equal(0);
          expect(res.body.unique_packages).to.equal(1);
          done(err);
        });
    });
  });
});
