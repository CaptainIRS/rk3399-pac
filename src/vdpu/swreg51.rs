#[doc = "Register `SWREG51` reader"]
pub type R = crate::R<Swreg51Spec>;
#[doc = "Register `SWREG51` writer"]
pub type W = crate::W<Swreg51Spec>;
#[doc = "Field `SW_STRM_LEN` reader - the stream data bytes number in input buffer.\n\nif the buffer size be given small than it required,hw will give an\n\ninterrupt,and then you should config again,and the stream start\n\naddress should be config also.\n\nVP6:\n\none picture/slice of the picture's should be included in the input\n\nbuffer\n\nH264/H263/MPEG*:\n\none slice of the picture's should be included in the input buffer\n\nJPEG:\n\n256bytes or onepicture should be included in the input buffer"]
pub type SwStrmLenR = crate::FieldReader<u32>;
#[doc = "Field `SW_STRM_LEN` writer - the stream data bytes number in input buffer.\n\nif the buffer size be given small than it required,hw will give an\n\ninterrupt,and then you should config again,and the stream start\n\naddress should be config also.\n\nVP6:\n\none picture/slice of the picture's should be included in the input\n\nbuffer\n\nH264/H263/MPEG*:\n\none slice of the picture's should be included in the input buffer\n\nJPEG:\n\n256bytes or onepicture should be included in the input buffer"]
pub type SwStrmLenW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SW_STRM_LEN_EXT` reader - The extension bit of sw_strm_len"]
pub type SwStrmLenExtR = crate::BitReader;
#[doc = "Field `SW_STRM_LEN_EXT` writer - The extension bit of sw_strm_len"]
pub type SwStrmLenExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_QP_INIT_VAL` reader - the qp(quantization parameter)'s Initial value"]
pub type SwQpInitValR = crate::FieldReader;
#[doc = "Field `SW_QP_INIT_VAL` writer - the qp(quantization parameter)'s Initial value"]
pub type SwQpInitValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:23 - the stream data bytes number in input buffer.\n\nif the buffer size be given small than it required,hw will give an\n\ninterrupt,and then you should config again,and the stream start\n\naddress should be config also.\n\nVP6:\n\none picture/slice of the picture's should be included in the input\n\nbuffer\n\nH264/H263/MPEG*:\n\none slice of the picture's should be included in the input buffer\n\nJPEG:\n\n256bytes or onepicture should be included in the input buffer"]
    #[inline(always)]
    pub fn sw_strm_len(&self) -> SwStrmLenR {
        SwStrmLenR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - The extension bit of sw_strm_len"]
    #[inline(always)]
    pub fn sw_strm_len_ext(&self) -> SwStrmLenExtR {
        SwStrmLenExtR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - the qp(quantization parameter)'s Initial value"]
    #[inline(always)]
    pub fn sw_qp_init_val(&self) -> SwQpInitValR {
        SwQpInitValR::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - the stream data bytes number in input buffer.\n\nif the buffer size be given small than it required,hw will give an\n\ninterrupt,and then you should config again,and the stream start\n\naddress should be config also.\n\nVP6:\n\none picture/slice of the picture's should be included in the input\n\nbuffer\n\nH264/H263/MPEG*:\n\none slice of the picture's should be included in the input buffer\n\nJPEG:\n\n256bytes or onepicture should be included in the input buffer"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strm_len(&mut self) -> SwStrmLenW<Swreg51Spec> {
        SwStrmLenW::new(self, 0)
    }
    #[doc = "Bit 24 - The extension bit of sw_strm_len"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strm_len_ext(&mut self) -> SwStrmLenExtW<Swreg51Spec> {
        SwStrmLenExtW::new(self, 24)
    }
    #[doc = "Bits 25:30 - the qp(quantization parameter)'s Initial value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_qp_init_val(&mut self) -> SwQpInitValW<Swreg51Spec> {
        SwQpInitValW::new(self, 25)
    }
}
#[doc = "the stream length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg51Spec;
impl crate::RegisterSpec for Swreg51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg51::R`](R) reader structure"]
impl crate::Readable for Swreg51Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg51::W`](W) writer structure"]
impl crate::Writable for Swreg51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG51 to value 0"]
impl crate::Resettable for Swreg51Spec {
    const RESET_VALUE: u32 = 0;
}
