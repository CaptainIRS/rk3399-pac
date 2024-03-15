#[doc = "Register `FUNC_EN_2` reader"]
pub type R = crate::R<FuncEn2Spec>;
#[doc = "Register `FUNC_EN_2` writer"]
pub type W = crate::W<FuncEn2Spec>;
#[doc = "Link symbol clock domain modules functions enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LsClkDomainFuncEnN {
    #[doc = "0: Disable the modules in link symbol clock domain. To reset the modules in link symbol clock domain, firmware must disable and enable this bit."]
    B0 = 0,
    #[doc = "1: Disable the modules in link symbol clock domain. To reset the modules in link symbol clock domain, firmware must disable and enable this bit."]
    B1 = 1,
}
impl From<LsClkDomainFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: LsClkDomainFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LS_CLK_DOMAIN_FUNC_EN_N` reader - Link symbol clock domain modules functions enable."]
pub type LsClkDomainFuncEnNR = crate::BitReader<LsClkDomainFuncEnN>;
impl LsClkDomainFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LsClkDomainFuncEnN {
        match self.bits {
            false => LsClkDomainFuncEnN::B0,
            true => LsClkDomainFuncEnN::B1,
        }
    }
    #[doc = "Disable the modules in link symbol clock domain. To reset the modules in link symbol clock domain, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LsClkDomainFuncEnN::B0
    }
    #[doc = "Disable the modules in link symbol clock domain. To reset the modules in link symbol clock domain, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LsClkDomainFuncEnN::B1
    }
}
#[doc = "Field `LS_CLK_DOMAIN_FUNC_EN_N` writer - Link symbol clock domain modules functions enable."]
pub type LsClkDomainFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, LsClkDomainFuncEnN>;
impl<'a, REG> LsClkDomainFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the modules in link symbol clock domain. To reset the modules in link symbol clock domain, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LsClkDomainFuncEnN::B0)
    }
    #[doc = "Disable the modules in link symbol clock domain. To reset the modules in link symbol clock domain, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LsClkDomainFuncEnN::B1)
    }
}
#[doc = "Serdes FIFO function enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SerdesFifoFuncEnN {
    #[doc = "0: Disable Serdes FIFO. To reset the serdes fifo, firmware must disable and enable this bit."]
    B0 = 0,
    #[doc = "1: Disable Serdes FIFO. To reset the serdes fifo, firmware must disable and enable this bit."]
    B1 = 1,
}
impl From<SerdesFifoFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: SerdesFifoFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SERDES_FIFO_FUNC_EN_N` reader - Serdes FIFO function enable."]
pub type SerdesFifoFuncEnNR = crate::BitReader<SerdesFifoFuncEnN>;
impl SerdesFifoFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SerdesFifoFuncEnN {
        match self.bits {
            false => SerdesFifoFuncEnN::B0,
            true => SerdesFifoFuncEnN::B1,
        }
    }
    #[doc = "Disable Serdes FIFO. To reset the serdes fifo, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SerdesFifoFuncEnN::B0
    }
    #[doc = "Disable Serdes FIFO. To reset the serdes fifo, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SerdesFifoFuncEnN::B1
    }
}
#[doc = "Field `SERDES_FIFO_FUNC_EN_N` writer - Serdes FIFO function enable."]
pub type SerdesFifoFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, SerdesFifoFuncEnN>;
impl<'a, REG> SerdesFifoFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Serdes FIFO. To reset the serdes fifo, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SerdesFifoFuncEnN::B0)
    }
    #[doc = "Disable Serdes FIFO. To reset the serdes fifo, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SerdesFifoFuncEnN::B1)
    }
}
#[doc = "AUX channel module function enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxFuncEnN {
    #[doc = "0: Disable AUX channel module."]
    B0 = 0,
    #[doc = "1: Disable AUX channel module."]
    B1 = 1,
}
impl From<AuxFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: AuxFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_FUNC_EN_N` reader - AUX channel module function enable."]
pub type AuxFuncEnNR = crate::BitReader<AuxFuncEnN>;
impl AuxFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxFuncEnN {
        match self.bits {
            false => AuxFuncEnN::B0,
            true => AuxFuncEnN::B1,
        }
    }
    #[doc = "Disable AUX channel module."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxFuncEnN::B0
    }
    #[doc = "Disable AUX channel module."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxFuncEnN::B1
    }
}
#[doc = "Field `AUX_FUNC_EN_N` writer - AUX channel module function enable."]
pub type AuxFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, AuxFuncEnN>;
impl<'a, REG> AuxFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable AUX channel module."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxFuncEnN::B0)
    }
    #[doc = "Disable AUX channel module."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxFuncEnN::B1)
    }
}
#[doc = "SSC module enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SscFuncEnN {
    #[doc = "0: Disable SSC module. To apply updated SSC parameters into SSC operation, firmware must disable and enable this bit."]
    B0 = 0,
    #[doc = "1: Disable SSC module. To apply updated SSC parameters into SSC operation, firmware must disable and enable this bit."]
    B1 = 1,
}
impl From<SscFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: SscFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSC_FUNC_EN_N` reader - SSC module enable."]
pub type SscFuncEnNR = crate::BitReader<SscFuncEnN>;
impl SscFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SscFuncEnN {
        match self.bits {
            false => SscFuncEnN::B0,
            true => SscFuncEnN::B1,
        }
    }
    #[doc = "Disable SSC module. To apply updated SSC parameters into SSC operation, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SscFuncEnN::B0
    }
    #[doc = "Disable SSC module. To apply updated SSC parameters into SSC operation, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SscFuncEnN::B1
    }
}
#[doc = "Field `SSC_FUNC_EN_N` writer - SSC module enable."]
pub type SscFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, SscFuncEnN>;
impl<'a, REG> SscFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable SSC module. To apply updated SSC parameters into SSC operation, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SscFuncEnN::B0)
    }
    #[doc = "Disable SSC module. To apply updated SSC parameters into SSC operation, firmware must disable and enable this bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SscFuncEnN::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Link symbol clock domain modules functions enable."]
    #[inline(always)]
    pub fn ls_clk_domain_func_en_n(&self) -> LsClkDomainFuncEnNR {
        LsClkDomainFuncEnNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serdes FIFO function enable."]
    #[inline(always)]
    pub fn serdes_fifo_func_en_n(&self) -> SerdesFifoFuncEnNR {
        SerdesFifoFuncEnNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUX channel module function enable."]
    #[inline(always)]
    pub fn aux_func_en_n(&self) -> AuxFuncEnNR {
        AuxFuncEnNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - SSC module enable."]
    #[inline(always)]
    pub fn ssc_func_en_n(&self) -> SscFuncEnNR {
        SscFuncEnNR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Link symbol clock domain modules functions enable."]
    #[inline(always)]
    #[must_use]
    pub fn ls_clk_domain_func_en_n(&mut self) -> LsClkDomainFuncEnNW<FuncEn2Spec> {
        LsClkDomainFuncEnNW::new(self, 0)
    }
    #[doc = "Bit 1 - Serdes FIFO function enable."]
    #[inline(always)]
    #[must_use]
    pub fn serdes_fifo_func_en_n(&mut self) -> SerdesFifoFuncEnNW<FuncEn2Spec> {
        SerdesFifoFuncEnNW::new(self, 1)
    }
    #[doc = "Bit 2 - AUX channel module function enable."]
    #[inline(always)]
    #[must_use]
    pub fn aux_func_en_n(&mut self) -> AuxFuncEnNW<FuncEn2Spec> {
        AuxFuncEnNW::new(self, 2)
    }
    #[doc = "Bit 7 - SSC module enable."]
    #[inline(always)]
    #[must_use]
    pub fn ssc_func_en_n(&mut self) -> SscFuncEnNW<FuncEn2Spec> {
        SscFuncEnNW::new(self, 7)
    }
}
#[doc = "Function Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_en_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_en_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncEn2Spec;
impl crate::RegisterSpec for FuncEn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_en_2::R`](R) reader structure"]
impl crate::Readable for FuncEn2Spec {}
#[doc = "`write(|w| ..)` method takes [`func_en_2::W`](W) writer structure"]
impl crate::Writable for FuncEn2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x87;
}
#[doc = "`reset()` method sets FUNC_EN_2 to value 0x87"]
impl crate::Resettable for FuncEn2Spec {
    const RESET_VALUE: u32 = 0x87;
}
