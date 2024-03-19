#[doc = "Register `GRF_SOC_CON8` reader"]
pub type R = crate::R<GrfSocCon8Spec>;
#[doc = "Register `GRF_SOC_CON8` writer"]
pub type W = crate::W<GrfSocCon8Spec>;
#[doc = "pcie test write control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieTestWrite {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PcieTestWrite> for bool {
    #[inline(always)]
    fn from(variant: PcieTestWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_TEST_WRITE` reader - pcie test write control"]
pub type PcieTestWriteR = crate::BitReader<PcieTestWrite>;
impl PcieTestWriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieTestWrite {
        match self.bits {
            false => PcieTestWrite::B0,
            true => PcieTestWrite::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieTestWrite::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieTestWrite::B1
    }
}
#[doc = "Field `PCIE_TEST_WRITE` writer - pcie test write control"]
pub type PcieTestWriteW<'a, REG> = crate::BitWriter<'a, REG, PcieTestWrite>;
impl<'a, REG> PcieTestWriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieTestWrite::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieTestWrite::B1)
    }
}
#[doc = "Field `PCIE_TEST_ADDR` reader - pci test address control"]
pub type PcieTestAddrR = crate::FieldReader;
#[doc = "Field `PCIE_TEST_ADDR` writer - pci test address control"]
pub type PcieTestAddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PCIE_TEST_I` reader - pci test input"]
pub type PcieTestIR = crate::FieldReader;
#[doc = "Field `PCIE_TEST_I` writer - pci test input"]
pub type PcieTestIW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2S0_SCLK_OE_N` reader - i2s0_sclk_oe_n bit control"]
pub type I2s0SclkOeNR = crate::FieldReader;
#[doc = "Field `I2S0_SCLK_OE_N` writer - i2s0_sclk_oe_n bit control"]
pub type I2s0SclkOeNW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pcie test write control"]
    #[inline(always)]
    pub fn pcie_test_write(&self) -> PcieTestWriteR {
        PcieTestWriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - pci test address control"]
    #[inline(always)]
    pub fn pcie_test_addr(&self) -> PcieTestAddrR {
        PcieTestAddrR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:10 - pci test input"]
    #[inline(always)]
    pub fn pcie_test_i(&self) -> PcieTestIR {
        PcieTestIR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - i2s0_sclk_oe_n bit control"]
    #[inline(always)]
    pub fn i2s0_sclk_oe_n(&self) -> I2s0SclkOeNR {
        I2s0SclkOeNR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pcie test write control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_test_write(&mut self) -> PcieTestWriteW<GrfSocCon8Spec> {
        PcieTestWriteW::new(self, 0)
    }
    #[doc = "Bits 1:6 - pci test address control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_test_addr(&mut self) -> PcieTestAddrW<GrfSocCon8Spec> {
        PcieTestAddrW::new(self, 1)
    }
    #[doc = "Bits 7:10 - pci test input"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_test_i(&mut self) -> PcieTestIW<GrfSocCon8Spec> {
        PcieTestIW::new(self, 7)
    }
    #[doc = "Bits 11:14 - i2s0_sclk_oe_n bit control"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_sclk_oe_n(&mut self) -> I2s0SclkOeNW<GrfSocCon8Spec> {
        I2s0SclkOeNW::new(self, 11)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon8Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon8Spec;
impl crate::RegisterSpec for GrfSocCon8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con8::R`](R) reader structure"]
impl crate::Readable for GrfSocCon8Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con8::W`](W) writer structure"]
impl crate::Writable for GrfSocCon8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON8 to value 0"]
impl crate::Resettable for GrfSocCon8Spec {
    const RESET_VALUE: u32 = 0;
}
