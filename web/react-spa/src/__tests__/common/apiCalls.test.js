import {post} from "../../common/apiCall";

jest.setTimeout(100000);

describe('apiCalls', function () {
  describe('post', function () {
    it('should return promise if response is in 200 range', async function () {
      const returnVal = {ok: true};
      jest.spyOn(global, 'fetch').mockResolvedValueOnce(returnVal);

      const mockData = {"key": "value"};
      const mockUrl = "/mock/url";

      const expectedReturnVal  = await post(mockUrl, mockData);

      expect(fetch).toHaveBeenCalledTimes(1);
      expect(fetch.mock.calls[0][0]).toBe("http://localhost:3000/api/mock/url");
      expect(fetch.mock.calls[0][1]).toMatchSnapshot()
      expect(expectedReturnVal).toBe(returnVal)
    });

    it('should throw error if the response is not in 200 range', async function (done) {
      const testError = "Test error";
      const returnVal = {ok: false, statusText: testError};
      jest.spyOn(global, 'fetch').mockResolvedValueOnce(returnVal);

      const mockData = {"key": "value"};
      const mockUrl = "/simulation/init";

      post(mockUrl, mockData)
        .catch((err) => {
          expect(err.message).toBe(testError);
          done()
        })
    });
  });
});