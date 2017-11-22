const expect = require('chai').expect;
const supertest = require('supertest');
const request = supertest('http://localhost:9636/v1');
const fs = require('fs');
const ps = require('process');

/*
 * There's some odd stuff going on in this file around the BLDR_FULL_TEST_RUN
 * env var, so let's discuss. When I wrote these tests and was iterating on
 * them in my local env, job logs weren't getting written out so I had to fake
 * it by writing out my own log file. When I ran these tests using test.sh,
 * like it would in CI, log files got written out. Running the tests via
 * test.sh is ideal for CI but horrible for local iteration while writing
 * tests, so test.sh exports BLDR_FULL_TEST_RUN=1 and we switch on that here.
 * My thought was trying to create something that covers us in CI but is also
 * not a pain to iterate on locally.
 */

let jobLogExpectations = function(res) {
  // Yes, I agree that this is a pathetic set of assertions for job logs but in
  // practice, the number of logs generated and their state varied wildly
  // between test runs and I couldn't make it deterministic so here we are.
  expect(res.body.start).to.equal(0);
};

describe('Jobs API', function() {
  describe('Scheduling jobs', function() {
    it('requires authentication', function(done) {
      request.post('/depot/pkgs/schedule/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires that you belong to the origin', function(done) {
      request.post('/depot/pkgs/schedule/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('only works for linux', function(done) {
      request.post('/depot/pkgs/schedule/neurosis/testapp?target=x86_64-windows')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(400)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('returns the group', function(done) {
      request.post('/depot/pkgs/schedule/neurosis/testapp')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        // JB TODO: this is wrong - it should be a 201, since we're creating
        // a job
        .expect(200)
        .end(function(err, res) {
          expect(res.body).to.not.be.empty;
          expect(res.body.state).to.equal('Queued');
          expect(res.body.project_name).to.equal('neurosis/testapp');
          global.neurosisJobGroup = res.body;
          done(err);
        });
    });

    it('pauses for a few seconds to let the job run', function(done) {
      this.timeout(5000);
      setTimeout(done, 4000);
    });
  });

  describe('Retrieving information about a job group', function() {
    it('requires a group id that is a u64', function(done) {
      request.get('/depot/pkgs/schedule/haha')
        .type('application/json')
        .accept('application/json')
        .expect(400)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('return the group', function(done) {
      request.get(`/depot/pkgs/schedule/${global.neurosisJobGroup['id']}`)
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body).to.not.be.empty;
          expect(['Queued','Dispatching']).to.include(res.body.state);
          expect(res.body.project_name).to.equal('neurosis/testapp');
          done(err);
        });
    });
  });

  describe('Retrieving information about every job group in an origin', function() {
    it('returns all of the groups', function(done) {
      request.get('/depot/pkgs/schedule/neurosis/status')
        .type('application/json')
        .accept('application/json')
        .expect(200)
        .end(function(err, res) {
          expect(res.body).to.not.be.empty;
          expect(res.body.length).to.equal(1);
          expect(['Queued','Dispatching']).to.include(res.body[0].state);
          expect(res.body[0].project_name).to.equal('neurosis/testapp');
          done(err);
        });
    });
  });

  describe('Listing all jobs for a project', function() {
    // JB TODO: should this require auth? seems like for public projects, no
    it('requires authentication', function(done) {
      request.get('/projects/neurosis/testapp/jobs')
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires membership in the origin that the project refers to', function(done) {
      request.get('/projects/neurosis/testapp/jobs')
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
      request.get('/projects/neurosis/testapp/jobs')
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.range_start).to.equal(0);
          expect(res.body.range_end).to.equal(0);
          expect(res.body.total_count).to.equal(1);
          expect(res.body.data.length).to.equal(1);
          expect(res.body.data[0].origin).to.equal('neurosis');
          expect(res.body.data[0].name).to.equal('testapp');
          global.neurosisTestappJob = res.body.data[0];
          done(err);
        });
    });
  });

  describe('Getting details of a job', function() {
    // JB TODO: should this require auth? for public projects, i don't think so
    it('requires authentication', function(done) {
      request.get(`/jobs/${global.neurosisTestappJob.id}`)
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires you are a member of the origin that the job belongs to', function(done) {
      request.get(`/jobs/${global.neurosisTestappJob.id}`)
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires a job id that is a u64', function(done) {
      request.get('/jobs/haha')
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
      request.get(`/jobs/${global.neurosisTestappJob.id}`)
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.id).to.equal(global.neurosisTestappJob.id);
          expect(res.body.origin).to.equal('neurosis');
          expect(res.body.name).to.equal('testapp');
          expect(res.body.channel).to.include('bldr-');
          done(err);
        });
    });
  });

  describe('Getting logs of a job', function() {
    // We need to fake a job log here because our test suite doesn't have all
    // the required deps to run a real build. Let's pretend that it did though.
    before(function() {
      if (!ps.env['BLDR_FULL_TEST_RUN']) {
        fs.writeFileSync(`/tmp/${global.neurosisTestappJob.id}.log`, 'This is a log file.');
      }
    });

    after(function() {
      if (!ps.env['BLDR_FULL_TEST_RUN']) {
        fs.unlinkSync(`/tmp/${global.neurosisTestappJob.id}.log`);
      }
    });

    // JB TODO: should this require auth? for public projects, i don't think so
    it('requires authentication', function(done) {
      request.get(`/jobs/${global.neurosisTestappJob.id}/log`)
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    // JB TODO: fix this
    // this is just weird - you can view logs as long as you're logged in, even
    // for origins that you don't belong to. job logs should be viewable w/o
    // auth for public projects and should only be viewable by members of the
    // origin for private projects
    it('requires you are logged in', function(done) {
      request.get(`/jobs/${global.neurosisTestappJob.id}/log`)
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(200)
        .end(function(err, res) {
          jobLogExpectations(res);
          done(err);
        });
    });

    describe('private projects', function() {
      it('sets the project to private first', function(done) {
        request.patch('/projects/neurosis/testapp/private')
          .accept('application/json')
          .set('Authorization', global.boboBearer)
          .expect(204)
          .end(function(err, res) {
            expect(res.text).to.be.empty;
            done(err);
          });
      });

      it('requires you are a member of the origin that the job belongs to when viewing logs for a private project', function(done) {
        request.get(`/jobs/${global.neurosisTestappJob.id}/log`)
          .accept('application/json')
          .set('Authorization', global.mystiqueBearer)
          .expect(403)
          .end(function(err, res) {
            expect(res.text).to.be.empty;
            done(err);
          });
      });

      it('shows the logs for a private project', function(done) {
        request.get(`/jobs/${global.neurosisTestappJob.id}/log`)
          .accept('application/json')
          .set('Authorization', global.boboBearer)
          .expect(200)
          .end(function(err, res) {
            jobLogExpectations(res);
            done(err);
          });
      });

      it('set the project back to public', function(done) {
        request.patch('/projects/neurosis/testapp/public')
          .accept('application/json')
          .set('Authorization', global.boboBearer)
          .expect(204)
          .end(function(err, res) {
            expect(res.text).to.be.empty;
            done(err);
          });
      });
    });

    it('requires a job id that is a u64', function(done) {
      request.get('/jobs/haha/log')
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(400)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('succeeds', function(done) {
      request.get(`/jobs/${global.neurosisTestappJob.id}/log`)
        .accept('application/json')
        .set('Authorization', global.boboBearer)
        .expect(200)
        .end(function(err, res) {
          jobLogExpectations(res);
          done(err);
        });
    });
  });


  describe('Promoting a job group', function() {
    it('requires authentication', function(done) {
      request.post(`/jobs/group/${global.neurosisJobGroup.id}/promote/bar`)
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    // It turns out this functionality is very hard to test because the jobs
    // that we've submitted don't finish by the time we get here.
    //
    // I'm skipping these for now but we should figure out a way to test these
    // at some point.
    it('requires you are a member of the origin that the job group belongs to', function(done) {
      this.skip();

      request.post(`/jobs/group/${global.neurosisJobGroup.id}/promote/bar`)
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .send({idents: ['neurosis/testapp']})
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires that the job group id is a u64');

    it('requires a valid JSON body');

    it('promotes every build in the group to the specified channel');
  });

  describe('Demoting a job group', function() {
    it('requires authentication', function(done) {
      request.post(`/jobs/group/${global.neurosisJobGroup.id}/demote/bar`)
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires you are a member of the origin that the job group belongs to');

    it('requires that the job group id is a u64');

    it('requires a valid JSON body');

    it('promotes every build in the group to the specified channel');
  });

  describe('Canceling a job group', function() {
    it('requires authentication', function(done) {
      request.post(`/jobs/group/${global.neurosisJobGroup.id}/cancel`)
        .type('application/json')
        .accept('application/json')
        .expect(401)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('requires you are a member of the origin that the job group belongs to', function(done) {
      request.post(`/jobs/group/${global.neurosisJobGroup.id}/cancel`)
        .type('application/json')
        .accept('application/json')
        .set('Authorization', global.mystiqueBearer)
        .expect(403)
        .end(function(err, res) {
          expect(res.text).to.be.empty;
          done(err);
        });
    });

    it('cancels the group', function(done) {
      request.post(`/jobs/group/${global.neurosisJobGroup.id}/cancel`)
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
