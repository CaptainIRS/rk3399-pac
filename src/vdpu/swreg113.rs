#[doc = "Register `SWREG113` reader"]
pub type R = crate::R<Swreg113Spec>;
#[doc = "Register `SWREG113` writer"]
pub type W = crate::W<Swreg113Spec>;
#[doc = "Field `H264_IDRP_ID` reader - instantaneous decoding refresh picture id"]
pub type H264IdrpIdR = crate::FieldReader<u16>;
#[doc = "Field `H264_IDRP_ID` writer - instantaneous decoding refresh picture id"]
pub type H264IdrpIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_MK_LEN` reader - the length of marking bits\n\nuse for decoded reference picture"]
pub type H264MkLenR = crate::FieldReader<u16>;
#[doc = "Field `H264_MK_LEN` writer - the length of marking bits\n\nuse for decoded reference picture"]
pub type H264MkLenW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - instantaneous decoding refresh picture id"]
    #[inline(always)]
    pub fn h264_idrp_id(&self) -> H264IdrpIdR {
        H264IdrpIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - the length of marking bits\n\nuse for decoded reference picture"]
    #[inline(always)]
    pub fn h264_mk_len(&self) -> H264MkLenR {
        H264MkLenR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - instantaneous decoding refresh picture id"]
    #[inline(always)]
    #[must_use]
    pub fn h264_idrp_id(&mut self) -> H264IdrpIdW<Swreg113Spec> {
        H264IdrpIdW::new(self, 0)
    }
    #[doc = "Bits 16:26 - the length of marking bits\n\nuse for decoded reference picture"]
    #[inline(always)]
    #[must_use]
    pub fn h264_mk_len(&mut self) -> H264MkLenW<Swreg113Spec> {
        H264MkLenW::new(self, 16)
    }
}
#[doc = "reference picture related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg113::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg113::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg113Spec;
impl crate::RegisterSpec for Swreg113Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg113::R`](R) reader structure"]
impl crate::Readable for Swreg113Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg113::W`](W) writer structure"]
impl crate::Writable for Swreg113Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG113 to value 0"]
impl crate::Resettable for Swreg113Spec {
    const RESET_VALUE: u32 = 0;
}
