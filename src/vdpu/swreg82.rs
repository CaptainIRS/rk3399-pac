#[doc = "Register `SWREG82` reader"]
pub type R = crate::R<Swreg82Spec>;
#[doc = "Register `SWREG82` writer"]
pub type W = crate::W<Swreg82Spec>;
#[doc = "Field `H264_NUM_REF_IDX12` reader - the number of referance pic index12"]
pub type H264NumRefIdx12R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX12` writer - the number of referance pic index12"]
pub type H264NumRefIdx12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX13` reader - the number of referance pic index13"]
pub type H264NumRefIdx13R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX13` writer - the number of referance pic index13"]
pub type H264NumRefIdx13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index12"]
    #[inline(always)]
    pub fn h264_num_ref_idx12(&self) -> H264NumRefIdx12R {
        H264NumRefIdx12R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index13"]
    #[inline(always)]
    pub fn h264_num_ref_idx13(&self) -> H264NumRefIdx13R {
        H264NumRefIdx13R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index12"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx12(&mut self) -> H264NumRefIdx12W<Swreg82Spec> {
        H264NumRefIdx12W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index13"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx13(&mut self) -> H264NumRefIdx13W<Swreg82Spec> {
        H264NumRefIdx13W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg82Spec;
impl crate::RegisterSpec for Swreg82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg82::R`](R) reader structure"]
impl crate::Readable for Swreg82Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg82::W`](W) writer structure"]
impl crate::Writable for Swreg82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG82 to value 0"]
impl crate::Resettable for Swreg82Spec {
    const RESET_VALUE: u32 = 0;
}
