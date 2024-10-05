pipeline {
	agent {
		docker {
			reuseNode true
			image 'rust:1.81'
		}
	}
	stages {
		stage('Build release binary') {
			steps {
				sh 'cargo build --release'
			}
		}
		stage('Move binary') {
			steps {
				sh 'mv ./target/release/thp /var/bin'
			}
		}
	}
}

