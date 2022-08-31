pipeline {
    agent {
        any
    }

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
        stage('Clippy') {
            steps {
                sh 'rustup component add clippy'
                sh 'cargo clippy --all'
            }
        }
        //stage('Format') {
        //    steps {
        //        // The build will fail if rustfmt thinks any changes are
        //        // required.
        //        sh "cargo fmt --all -- --write-mode diff"
        //    }
        //}
        stage('Doc') {
            steps {
                sh "cargo doc"
            }
        }
    }
}
