#[doc = "Register `SWREG81` reader"]
pub type R = crate::R<Swreg81Spec>;
#[doc = "Register `SWREG81` writer"]
pub type W = crate::W<Swreg81Spec>;
#[doc = "Field `H264_NUM_REF_IDX10` reader - the number of referance pic index10"]
pub type H264NumRefIdx10R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX10` writer - the number of referance pic index10"]
pub type H264NumRefIdx10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX11` reader - the number of referance pic index11"]
pub type H264NumRefIdx11R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX11` writer - the number of referance pic index11"]
pub type H264NumRefIdx11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index10"]
    #[inline(always)]
    pub fn h264_num_ref_idx10(&self) -> H264NumRefIdx10R {
        H264NumRefIdx10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index11"]
    #[inline(always)]
    pub fn h264_num_ref_idx11(&self) -> H264NumRefIdx11R {
        H264NumRefIdx11R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index10"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx10(&mut self) -> H264NumRefIdx10W<Swreg81Spec> {
        H264NumRefIdx10W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index11"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx11(&mut self) -> H264NumRefIdx11W<Swreg81Spec> {
        H264NumRefIdx11W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg81Spec;
impl crate::RegisterSpec for Swreg81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg81::R`](R) reader structure"]
impl crate::Readable for Swreg81Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg81::W`](W) writer structure"]
impl crate::Writable for Swreg81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG81 to value 0"]
impl crate::Resettable for Swreg81Spec {
    const RESET_VALUE: u32 = 0;
}
