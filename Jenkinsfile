pipeline {
    agent any;

    stages {
        stage('Build') {
            steps {
                sh 'cargo version'
                sh 'cargo build'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
        stage('Doc') {
            steps {
                sh "cargo doc --no-deps"
            }
        }
    }
}
