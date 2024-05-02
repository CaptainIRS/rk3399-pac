#[doc = "Register `DPCC_BPT_NUMBER` reader"]
pub type R = crate::R<DpccBptNumberSpec>;
#[doc = "Register `DPCC_BPT_NUMBER` writer"]
pub type W = crate::W<DpccBptNumberSpec>;
#[doc = "Field `bp_number` reader - Number of current Bad Pixel entries in bad pixel table\n\n(BPT)"]
pub type BpNumberR = crate::FieldReader<u16>;
#[doc = "Field `bp_number` writer - Number of current Bad Pixel entries in bad pixel table\n\n(BPT)"]
pub type BpNumberW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of current Bad Pixel entries in bad pixel table\n\n(BPT)"]
    #[inline(always)]
    pub fn bp_number(&self) -> BpNumberR {
        BpNumberR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of current Bad Pixel entries in bad pixel table\n\n(BPT)"]
    #[inline(always)]
    #[must_use]
    pub fn bp_number(&mut self) -> BpNumberW<DpccBptNumberSpec> {
        BpNumberW::new(self, 0)
    }
}
#[doc = "Number of entries for bad pixel table (table based correction)\n\nNote: bit width of bp_number depends on size of BP RAM which is defined during chip \n\n\n\nsynthesis \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_number::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_number::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccBptNumberSpec;
impl crate::RegisterSpec for DpccBptNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_bpt_number::R`](R) reader structure"]
impl crate::Readable for DpccBptNumberSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_bpt_number::W`](W) writer structure"]
impl crate::Writable for DpccBptNumberSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_BPT_NUMBER to value 0"]
impl crate::Resettable for DpccBptNumberSpec {
    const RESET_VALUE: u32 = 0;
}
