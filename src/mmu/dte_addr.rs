#[doc = "Register `DTE_ADDR` reader"]
pub type R = crate::R<DteAddrSpec>;
#[doc = "Register `DTE_ADDR` writer"]
pub type W = crate::W<DteAddrSpec>;
#[doc = "Field `MMU_DTE_ADDR` reader - page table address"]
pub type MmuDteAddrR = crate::FieldReader<u32>;
#[doc = "Field `MMU_DTE_ADDR` writer - page table address"]
pub type MmuDteAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - page table address"]
    #[inline(always)]
    pub fn mmu_dte_addr(&self) -> MmuDteAddrR {
        MmuDteAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - page table address"]
    #[inline(always)]
    #[must_use]
    pub fn mmu_dte_addr(&mut self) -> MmuDteAddrW<DteAddrSpec> {
        MmuDteAddrW::new(self, 0)
    }
}
#[doc = "MMU current page table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dte_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dte_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DteAddrSpec;
impl crate::RegisterSpec for DteAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dte_addr::R`](R) reader structure"]
impl crate::Readable for DteAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dte_addr::W`](W) writer structure"]
impl crate::Writable for DteAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTE_ADDR to value 0"]
impl crate::Resettable for DteAddrSpec {
    const RESET_VALUE: u32 = 0;
}
