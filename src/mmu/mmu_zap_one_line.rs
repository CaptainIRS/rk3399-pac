#[doc = "Register `MMU_ZAP_ONE_LINE` reader"]
pub type R = crate::R<MmuZapOneLineSpec>;
#[doc = "Register `MMU_ZAP_ONE_LINE` writer"]
pub type W = crate::W<MmuZapOneLineSpec>;
#[doc = "Field `MMU_ZAP_ONE_LINE` reader - address to be invalidated from the page table\n\ncache."]
pub type MmuZapOneLineR = crate::FieldReader<u32>;
#[doc = "Field `MMU_ZAP_ONE_LINE` writer - address to be invalidated from the page table\n\ncache."]
pub type MmuZapOneLineW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - address to be invalidated from the page table\n\ncache."]
    #[inline(always)]
    pub fn mmu_zap_one_line(&self) -> MmuZapOneLineR {
        MmuZapOneLineR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - address to be invalidated from the page table\n\ncache."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_zap_one_line(&mut self) -> MmuZapOneLineW<MmuZapOneLineSpec> {
        MmuZapOneLineW::new(self, 0)
    }
}
#[doc = "MMU zap cache line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_zap_one_line::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_zap_one_line::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuZapOneLineSpec;
impl crate::RegisterSpec for MmuZapOneLineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_zap_one_line::R`](R) reader structure"]
impl crate::Readable for MmuZapOneLineSpec {}
#[doc = "`write(|w| ..)` method takes [`mmu_zap_one_line::W`](W) writer structure"]
impl crate::Writable for MmuZapOneLineSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMU_ZAP_ONE_LINE to value 0"]
impl crate::Resettable for MmuZapOneLineSpec {
    const RESET_VALUE: u32 = 0;
}
