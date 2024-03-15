#[doc = "Register `BUFFER_DATA_CTL` reader"]
pub type R = crate::R<BufferDataCtlSpec>;
#[doc = "Register `BUFFER_DATA_CTL` writer"]
pub type W = crate::W<BufferDataCtlSpec>;
#[doc = "Field `BUF_DATA_COUNT` reader - The counts of data AUX CH buffer have. This bit's type is RO."]
pub type BufDataCountR = crate::FieldReader;
#[doc = "Field `BUF_HAVE_DATA` reader - 0:buffer have data,1:buffer have not data"]
pub type BufHaveDataR = crate::BitReader;
#[doc = "Field `BUF_CLR` reader - Write 1 to this bit to clear AUX CH data buffer (BUF_DATA_0 ~ BUF_DATA_15). Always read back 0 from this bit. This bit's type is R/W. This bit is self cleared. Note: For the write operation, set this bit to 1 before writing data to BUF_DATA_0~15. And for READ operation, this bit has only to be set before starting data transfer by setting AUX_EN."]
pub type BufClrR = crate::BitReader;
#[doc = "Field `BUF_CLR` writer - Write 1 to this bit to clear AUX CH data buffer (BUF_DATA_0 ~ BUF_DATA_15). Always read back 0 from this bit. This bit's type is R/W. This bit is self cleared. Note: For the write operation, set this bit to 1 before writing data to BUF_DATA_0~15. And for READ operation, this bit has only to be set before starting data transfer by setting AUX_EN."]
pub type BufClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The counts of data AUX CH buffer have. This bit's type is RO."]
    #[inline(always)]
    pub fn buf_data_count(&self) -> BufDataCountR {
        BufDataCountR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0:buffer have data,1:buffer have not data"]
    #[inline(always)]
    pub fn buf_have_data(&self) -> BufHaveDataR {
        BufHaveDataR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to this bit to clear AUX CH data buffer (BUF_DATA_0 ~ BUF_DATA_15). Always read back 0 from this bit. This bit's type is R/W. This bit is self cleared. Note: For the write operation, set this bit to 1 before writing data to BUF_DATA_0~15. And for READ operation, this bit has only to be set before starting data transfer by setting AUX_EN."]
    #[inline(always)]
    pub fn buf_clr(&self) -> BufClrR {
        BufClrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Write 1 to this bit to clear AUX CH data buffer (BUF_DATA_0 ~ BUF_DATA_15). Always read back 0 from this bit. This bit's type is R/W. This bit is self cleared. Note: For the write operation, set this bit to 1 before writing data to BUF_DATA_0~15. And for READ operation, this bit has only to be set before starting data transfer by setting AUX_EN."]
    #[inline(always)]
    #[must_use]
    pub fn buf_clr(&mut self) -> BufClrW<BufferDataCtlSpec> {
        BufClrW::new(self, 7)
    }
}
#[doc = "DP Buffer Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffer_data_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer_data_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufferDataCtlSpec;
impl crate::RegisterSpec for BufferDataCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffer_data_ctl::R`](R) reader structure"]
impl crate::Readable for BufferDataCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`buffer_data_ctl::W`](W) writer structure"]
impl crate::Writable for BufferDataCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x80;
}
#[doc = "`reset()` method sets BUFFER_DATA_CTL to value 0"]
impl crate::Resettable for BufferDataCtlSpec {
    const RESET_VALUE: u32 = 0;
}
