FROM smartcontract/builder:1.0.1

# The integration tests need jq
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y jq

# Install goverage to capture go test coverage
RUN go get -u github.com/haya14busa/goverage
RUN curl -L https://codeclimate.com/downloads/test-reporter/test-reporter-latest-linux-amd64 \
      > "/usr/local/bin/cc-test-reporter" \
      && chmod +x "/usr/local/bin/cc-test-reporter"

# Create the project working directory in the full GOPATH
RUN mkdir -p /go/src/github.com/smartcontractkit/chainlink/
WORKDIR /go/src/github.com/smartcontractkit/chainlink

# Do dependency installs first, since these will change less than the full
# source tree and can get cached
ADD Gopkg.* ./
RUN dep ensure -vendor-only
ADD package.json yarn.lock ./
RUN yarn install

# Copy in full source
ADD . .

CMD ["/bin/bash"]
