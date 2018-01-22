const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');

// These magic values correspond to the testpp repo in the habitat-sh org
const installationId = 56940;
const repoId = 114932712;
const projectCreatePayload = {
  origin: 'neurosis',
  plan_path: 'plan.sh',
  installation_id: installationId,
  repo_id: repoId
};

let projectExpectations = function(res) {
  expect(res.body.id).to.not.be.empty;
  expect(res.body.origin_id).to.equal(global.originNeurosis.id.toString());
  expect(res.body.origin_name).to.equal('neurosis');
  expect(res.body.package_name).to.equal('testapp');
  expect(res.body.name).to.equal('neurosis/testapp');
  expect(res.body.plan_path).to.equal('plan.sh');
  expect(res.body.owner_id).to.equal(global.sessionBobo.id);
  expect(res.body.vcs_type).to.equal('git');
  expect(res.body.vcs_data).to.equal('https://github.com/habitat-sh/testapp.git')
  expect(res.body.vcs_installation_id).to.equal(installationId.toString());
  expect(res.body.visibility).to.equal('public');
};

describe('Projects API', function() {
  describe('Creating a project', function() {
    it('requires authentication', function(done) {
      request.post('/projects')
        .type('application/json')
        .accept('application/json')
        .send(projectCreatePayload)
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.post('/projects')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .send(projectCreatePayload)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires a properly formatted payload', function(done) {
      request.post('/projects')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send({
          haha: 'lulz'
        })
        .expect(422)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      this.timeout(5000);
      request.post('/projects')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send(projectCreatePayload)
        .expect(201)
        .end(function(err, res) {
          projectExpectations(res);
          done(err);
        });
    });
  });

  describe('Retrieving a project', function() {
    it('requires authentication', function(done) {
      request.get('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.get('/projects/neurosis/testapp')
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
      request.get('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          projectExpectations(res);
          done(err);
        });
    });
  });

  describe('Listing all projects in an origin', function() {
    it('requires authentication', function(done) {
      request.get('/projects/neurosis')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.get('/projects/neurosis')
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
      request.get('/projects/neurosis')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.length).to.equal(1);
          expect(res.body[0]).to.equal('testapp');
          done(err);
        });
    });
  });

  describe('Editing a project', function() {
    it('requires authentication', function(done) {
      request.put('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.put('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires a properly formatted payload', function(done) {
      request.put('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send({
          haha: 'lulz'
        })
        .expect(422)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      this.timeout(5000);
      request.put('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send({
          plan_path: 'awesome/plan.sh',
          installation_id: installationId,
          repo_id: repoId
        })
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('reflects the new changes when viewing it again', function(done) {
      request.get('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.id).to.not.be.empty;
          expect(res.body.origin_id).to.equal(global.originNeurosis.id.toString());
          expect(res.body.origin_name).to.equal('neurosis');
          expect(res.body.package_name).to.equal('testapp');
          expect(res.body.name).to.equal('neurosis/testapp');
          expect(res.body.plan_path).to.equal('awesome/plan.sh');
          expect(res.body.owner_id).to.equal(global.sessionBobo.id);
          expect(res.body.vcs_type).to.equal('git');
          expect(res.body.vcs_data).to.equal('https://github.com/habitat-sh/testapp.git')
          expect(res.body.vcs_installation_id).to.equal(installationId.toString());
          expect(res.body.visibility).to.equal('public');
          done(err);
        });
    });
  });

  describe('Toggling the privacy of a project', function() {
    it('requires authentication', function(done) {
      request.patch('/projects/neurosis/testapp/private')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.patch('/projects/neurosis/testapp/private')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires that you set it to a known visibility setting', function(done) {
      request.patch('/projects/neurosis/testapp/lulz')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(400)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('does not allow you to set hidden manually', function(done) {
      request.patch('/projects/neurosis/testapp/hidden')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(400)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.patch('/projects/neurosis/testapp/private')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Deleting a project', function() {
    it('requires authentication', function(done) {
      request.delete('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.delete('/projects/neurosis/testapp')
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
      request.delete('/projects/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('creates another project so that other tests dont fail', function(done) {
      this.timeout(5000);
      request.post('/projects')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send(projectCreatePayload)
        .expect(201)
        .end(function(err, res) {
          projectExpectations(res);
          done(err);
        });
    });
  });
});

describe('Project integrations API', function() {
  describe('Creating a project integration', function() {
    it('requires authentication', function(done) {
      request.put('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .send({
          fun: 'stuff',
          awesome: true,
          numbers: 123
        })
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.put('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .send({
          fun: 'stuff',
          awesome: true,
          numbers: 123
        })
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires a JSON body', function(done) {
      request.put('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send('this is not JSON')
        .expect(400)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.put('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .send({
          fun: 'stuff',
          awesome: true,
          numbers: 123
        })
        // JB TODO: this is wrong - it should be a 201
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });

  describe('Retrieving a project integration', function() {
    it('requires authentication', function(done) {
      request.get('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.get('/projects/neurosis/testapp/integrations/docker/default')
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
      request.get('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body).to.deep.equal({
            fun: 'stuff',
            awesome: true,
            numbers: 123
          });
          done(err);
        });
    });
  });

  describe('Deleting a project integration', function() {
    it('requires authentication', function(done) {
      request.delete('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.delete('/projects/neurosis/testapp/integrations/docker/default')
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
      request.delete('/projects/neurosis/testapp/integrations/docker/default')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(204)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });
  });
});
