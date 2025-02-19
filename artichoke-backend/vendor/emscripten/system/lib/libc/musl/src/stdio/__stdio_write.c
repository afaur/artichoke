#include "stdio_impl.h"
#include <sys/uio.h>
#ifdef __EMSCRIPTEN__
#include <wasi/wasi.h>
#endif

size_t __stdio_write(FILE *f, const unsigned char *buf, size_t len)
{
	struct iovec iovs[2] = {
		{ .iov_base = f->wbase, .iov_len = f->wpos-f->wbase },
		{ .iov_base = (void *)buf, .iov_len = len }
	};
	struct iovec *iov = iovs;
	size_t rem = iov[0].iov_len + iov[1].iov_len;
	int iovcnt = 2;
	ssize_t cnt;
	for (;;) {
#if __EMSCRIPTEN__
		size_t num;
		__wasi_errno_t err = __wasi_fd_write(f->fd, (struct __wasi_ciovec_t*)iov, iovcnt, &num);
		if (err != __WASI_ESUCCESS) {
				num = -1;
		}
		cnt = num;
#else
		cnt = syscall(SYS_writev, f->fd, iov, iovcnt);
#endif
		if (cnt == rem) {
			f->wend = f->buf + f->buf_size;
			f->wpos = f->wbase = f->buf;
			return len;
		}
		if (cnt < 0) {
			f->wpos = f->wbase = f->wend = 0;
			f->flags |= F_ERR;
			return iovcnt == 2 ? 0 : len-iov[0].iov_len;
		}
		rem -= cnt;
		if (cnt > iov[0].iov_len) {
			cnt -= iov[0].iov_len;
			iov++; iovcnt--;
		}
		iov[0].iov_base = (char *)iov[0].iov_base + cnt;
		iov[0].iov_len -= cnt;
	}
}
