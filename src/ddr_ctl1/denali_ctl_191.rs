#[doc = "Register `DENALI_CTL_191` reader"]
pub type R = crate::R<DenaliCtl191Spec>;
#[doc = "Register `DENALI_CTL_191` writer"]
pub type W = crate::W<DenaliCtl191Spec>;
#[doc = "Field `COL_DIFF` reader - Difference between number of column pins available and number being used."]
pub type ColDiffR = crate::FieldReader;
#[doc = "Field `COL_DIFF` writer - Difference between number of column pins available and number being used."]
pub type ColDiffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BANK_START_BIT` reader - Defines the LSbit of the bank address within the page of the user address when the BANK_ADDR_INTLV_EN parameter is set."]
pub type BankStartBitR = crate::FieldReader;
#[doc = "Field `BANK_START_BIT` writer - Defines the LSbit of the bank address within the page of the user address when the BANK_ADDR_INTLV_EN parameter is set."]
pub type BankStartBitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BANK_ADDR_INTLV_EN` reader - Enables the capability to interleave the bank address within the row address bits. Set to 1 to enable."]
pub type BankAddrIntlvEnR = crate::BitReader;
#[doc = "Field `BANK_ADDR_INTLV_EN` writer - Enables the capability to interleave the bank address within the row address bits. Set to 1 to enable."]
pub type BankAddrIntlvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APREBIT` reader - Location of the auto pre-charge bit in the DRAM address."]
pub type AprebitR = crate::FieldReader;
#[doc = "Field `APREBIT` writer - Location of the auto pre-charge bit in the DRAM address."]
pub type AprebitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Difference between number of column pins available and number being used."]
    #[inline(always)]
    pub fn col_diff(&self) -> ColDiffR {
        ColDiffR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Defines the LSbit of the bank address within the page of the user address when the BANK_ADDR_INTLV_EN parameter is set."]
    #[inline(always)]
    pub fn bank_start_bit(&self) -> BankStartBitR {
        BankStartBitR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enables the capability to interleave the bank address within the row address bits. Set to 1 to enable."]
    #[inline(always)]
    pub fn bank_addr_intlv_en(&self) -> BankAddrIntlvEnR {
        BankAddrIntlvEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Location of the auto pre-charge bit in the DRAM address."]
    #[inline(always)]
    pub fn aprebit(&self) -> AprebitR {
        AprebitR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Difference between number of column pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn col_diff(&mut self) -> ColDiffW<DenaliCtl191Spec> {
        ColDiffW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Defines the LSbit of the bank address within the page of the user address when the BANK_ADDR_INTLV_EN parameter is set."]
    #[inline(always)]
    #[must_use]
    pub fn bank_start_bit(&mut self) -> BankStartBitW<DenaliCtl191Spec> {
        BankStartBitW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the capability to interleave the bank address within the row address bits. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bank_addr_intlv_en(&mut self) -> BankAddrIntlvEnW<DenaliCtl191Spec> {
        BankAddrIntlvEnW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Location of the auto pre-charge bit in the DRAM address."]
    #[inline(always)]
    #[must_use]
    pub fn aprebit(&mut self) -> AprebitW<DenaliCtl191Spec> {
        AprebitW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_191::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_191::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl191Spec;
impl crate::RegisterSpec for DenaliCtl191Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_191::R`](R) reader structure"]
impl crate::Readable for DenaliCtl191Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_191::W`](W) writer structure"]
impl crate::Writable for DenaliCtl191Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_191 to value 0x0a00_0000"]
impl crate::Resettable for DenaliCtl191Spec {
    const RESET_VALUE: u32 = 0x0a00_0000;
}
