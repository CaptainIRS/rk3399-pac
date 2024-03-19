#[doc = "Register `DP_AUX` reader"]
pub type R = crate::R<DpAuxSpec>;
#[doc = "Register `DP_AUX` writer"]
pub type W = crate::W<DpAuxSpec>;
#[doc = "AUX CH impedance control bits: \n\nonly control TX impedance\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AuxTerm {
    #[doc = "0: 500ohm"]
    B00 = 0,
    #[doc = "1: 250ohm"]
    B01 = 1,
    #[doc = "2: 100ohm"]
    B10 = 2,
    #[doc = "3: 50ohm"]
    B11 = 3,
}
impl From<AuxTerm> for u8 {
    #[inline(always)]
    fn from(variant: AuxTerm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AuxTerm {
    type Ux = u8;
}
#[doc = "Field `AUX_TERM` reader - AUX CH impedance control bits: \n\nonly control TX impedance"]
pub type AuxTermR = crate::FieldReader<AuxTerm>;
impl AuxTermR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTerm {
        match self.bits {
            0 => AuxTerm::B00,
            1 => AuxTerm::B01,
            2 => AuxTerm::B10,
            3 => AuxTerm::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "500ohm"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AuxTerm::B00
    }
    #[doc = "250ohm"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AuxTerm::B01
    }
    #[doc = "100ohm"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AuxTerm::B10
    }
    #[doc = "50ohm"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AuxTerm::B11
    }
}
#[doc = "Field `AUX_TERM` writer - AUX CH impedance control bits: \n\nonly control TX impedance"]
pub type AuxTermW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AuxTerm>;
impl<'a, REG> AuxTermW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "500ohm"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTerm::B00)
    }
    #[doc = "250ohm"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTerm::B01)
    }
    #[doc = "100ohm"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTerm::B10)
    }
    #[doc = "50ohm"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTerm::B11)
    }
}
#[doc = "AUX TX enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpAuxEn {
    #[doc = "0: AUX CH configured as RX"]
    B0 = 0,
    #[doc = "1: AUX CH configured as TX"]
    B1 = 1,
}
impl From<DpAuxEn> for bool {
    #[inline(always)]
    fn from(variant: DpAuxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_AUX_EN` reader - AUX TX enable"]
pub type DpAuxEnR = crate::BitReader<DpAuxEn>;
impl DpAuxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpAuxEn {
        match self.bits {
            false => DpAuxEn::B0,
            true => DpAuxEn::B1,
        }
    }
    #[doc = "AUX CH configured as RX"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpAuxEn::B0
    }
    #[doc = "AUX CH configured as TX"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpAuxEn::B1
    }
}
#[doc = "AUX RX CM voltage control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpAuxCommonMode {
    #[doc = "0: AUX CH use VCC1/2 as CM voltage (have static current consumption)"]
    B0 = 0,
    #[doc = "1: use VCC1 as CM voltage"]
    B1 = 1,
}
impl From<DpAuxCommonMode> for bool {
    #[inline(always)]
    fn from(variant: DpAuxCommonMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_AUX_COMMON_MODE` reader - AUX RX CM voltage control"]
pub type DpAuxCommonModeR = crate::BitReader<DpAuxCommonMode>;
impl DpAuxCommonModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpAuxCommonMode {
        match self.bits {
            false => DpAuxCommonMode::B0,
            true => DpAuxCommonMode::B1,
        }
    }
    #[doc = "AUX CH use VCC1/2 as CM voltage (have static current consumption)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpAuxCommonMode::B0
    }
    #[doc = "use VCC1 as CM voltage"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpAuxCommonMode::B1
    }
}
#[doc = "Field `DP_AUX_COMMON_MODE` writer - AUX RX CM voltage control"]
pub type DpAuxCommonModeW<'a, REG> = crate::BitWriter1C<'a, REG, DpAuxCommonMode>;
impl<'a, REG> DpAuxCommonModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX CH use VCC1/2 as CM voltage (have static current consumption)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpAuxCommonMode::B0)
    }
    #[doc = "use VCC1 as CM voltage"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpAuxCommonMode::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - AUX CH impedance control bits: \n\nonly control TX impedance"]
    #[inline(always)]
    pub fn aux_term(&self) -> AuxTermR {
        AuxTermR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - AUX TX enable"]
    #[inline(always)]
    pub fn dp_aux_en(&self) -> DpAuxEnR {
        DpAuxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUX RX CM voltage control"]
    #[inline(always)]
    pub fn dp_aux_common_mode(&self) -> DpAuxCommonModeR {
        DpAuxCommonModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AUX CH impedance control bits: \n\nonly control TX impedance"]
    #[inline(always)]
    #[must_use]
    pub fn aux_term(&mut self) -> AuxTermW<DpAuxSpec> {
        AuxTermW::new(self, 0)
    }
    #[doc = "Bit 4 - AUX RX CM voltage control"]
    #[inline(always)]
    #[must_use]
    pub fn dp_aux_common_mode(&mut self) -> DpAuxCommonModeW<DpAuxSpec> {
        DpAuxCommonModeW::new(self, 4)
    }
}
#[doc = "Aux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_aux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_aux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpAuxSpec;
impl crate::RegisterSpec for DpAuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_aux::R`](R) reader structure"]
impl crate::Readable for DpAuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_aux::W`](W) writer structure"]
impl crate::Writable for DpAuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets DP_AUX to value 0x07"]
impl crate::Resettable for DpAuxSpec {
    const RESET_VALUE: u32 = 0x07;
}
