#[doc = "Register `CHNSRn` reader"]
pub type R = crate::R<ChnsrnSpec>;
#[doc = "Register `CHNSRn` writer"]
pub type W = crate::W<ChnsrnSpec>;
#[doc = "Field `CHNSR_SUB_0` reader - Channel Status Subframe 0\n\nChannel Status Bit for Subframe 0"]
pub type ChnsrSub0R = crate::FieldReader<u16>;
#[doc = "Field `CHNSR_SUB_0` writer - Channel Status Subframe 0\n\nChannel Status Bit for Subframe 0"]
pub type ChnsrSub0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CHNSR_SUB_1` reader - Channel Status Subframe 1\n\nChannel Status Bit for Subframe 1"]
pub type ChnsrSub1R = crate::FieldReader<u16>;
#[doc = "Field `CHNSR_SUB_1` writer - Channel Status Subframe 1\n\nChannel Status Bit for Subframe 1"]
pub type ChnsrSub1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel Status Subframe 0\n\nChannel Status Bit for Subframe 0"]
    #[inline(always)]
    pub fn chnsr_sub_0(&self) -> ChnsrSub0R {
        ChnsrSub0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Channel Status Subframe 1\n\nChannel Status Bit for Subframe 1"]
    #[inline(always)]
    pub fn chnsr_sub_1(&self) -> ChnsrSub1R {
        ChnsrSub1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Status Subframe 0\n\nChannel Status Bit for Subframe 0"]
    #[inline(always)]
    #[must_use]
    pub fn chnsr_sub_0(&mut self) -> ChnsrSub0W<ChnsrnSpec> {
        ChnsrSub0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Channel Status Subframe 1\n\nChannel Status Bit for Subframe 1"]
    #[inline(always)]
    #[must_use]
    pub fn chnsr_sub_1(&mut self) -> ChnsrSub1W<ChnsrnSpec> {
        ChnsrSub1W::new(self, 16)
    }
}
#[doc = "Channel Status Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chnsrn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chnsrn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnsrnSpec;
impl crate::RegisterSpec for ChnsrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnsrn::R`](R) reader structure"]
impl crate::Readable for ChnsrnSpec {}
#[doc = "`write(|w| ..)` method takes [`chnsrn::W`](W) writer structure"]
impl crate::Writable for ChnsrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHNSRn to value 0"]
impl crate::Resettable for ChnsrnSpec {
    const RESET_VALUE: u32 = 0;
}
