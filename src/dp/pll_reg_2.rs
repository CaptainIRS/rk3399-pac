#[doc = "Register `PLL_REG_2` reader"]
pub type R = crate::R<PllReg2Spec>;
#[doc = "Register `PLL_REG_2` writer"]
pub type W = crate::W<PllReg2Spec>;
#[doc = "v2i current select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V2iCurrentSel {
    #[doc = "0: adding 4.5mA current"]
    B00 = 0,
    #[doc = "1: adding 4.5mA current"]
    B01 = 1,
    #[doc = "2: adding 4.5mA current"]
    B10 = 2,
    #[doc = "3: adding 4.5mA current"]
    B11 = 3,
}
impl From<V2iCurrentSel> for u8 {
    #[inline(always)]
    fn from(variant: V2iCurrentSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for V2iCurrentSel {
    type Ux = u8;
}
#[doc = "Field `V2I_CURRENT_SEL` reader - v2i current select"]
pub type V2iCurrentSelR = crate::FieldReader<V2iCurrentSel>;
impl V2iCurrentSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V2iCurrentSel {
        match self.bits {
            0 => V2iCurrentSel::B00,
            1 => V2iCurrentSel::B01,
            2 => V2iCurrentSel::B10,
            3 => V2iCurrentSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == V2iCurrentSel::B00
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == V2iCurrentSel::B01
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == V2iCurrentSel::B10
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == V2iCurrentSel::B11
    }
}
#[doc = "Field `V2I_CURRENT_SEL` writer - v2i current select"]
pub type V2iCurrentSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, V2iCurrentSel>;
impl<'a, REG> V2iCurrentSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(V2iCurrentSel::B00)
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(V2iCurrentSel::B01)
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(V2iCurrentSel::B10)
    }
    #[doc = "adding 4.5mA current"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(V2iCurrentSel::B11)
    }
}
#[doc = "charge pump current select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChgPumpCurrentSel {
    #[doc = "0: 10u"]
    B00 = 0,
    #[doc = "2: 10u"]
    B10 = 2,
    #[doc = "3: 10u"]
    B11 = 3,
}
impl From<ChgPumpCurrentSel> for u8 {
    #[inline(always)]
    fn from(variant: ChgPumpCurrentSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ChgPumpCurrentSel {
    type Ux = u8;
}
#[doc = "Field `CHG_PUMP_CURRENT_SEL` reader - charge pump current select"]
pub type ChgPumpCurrentSelR = crate::FieldReader<ChgPumpCurrentSel>;
impl ChgPumpCurrentSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ChgPumpCurrentSel> {
        match self.bits {
            0 => Some(ChgPumpCurrentSel::B00),
            2 => Some(ChgPumpCurrentSel::B10),
            3 => Some(ChgPumpCurrentSel::B11),
            _ => None,
        }
    }
    #[doc = "10u"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ChgPumpCurrentSel::B00
    }
    #[doc = "10u"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ChgPumpCurrentSel::B10
    }
    #[doc = "10u"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ChgPumpCurrentSel::B11
    }
}
#[doc = "Field `CHG_PUMP_CURRENT_SEL` writer - charge pump current select"]
pub type ChgPumpCurrentSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ChgPumpCurrentSel>;
impl<'a, REG> ChgPumpCurrentSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10u"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpCurrentSel::B00)
    }
    #[doc = "10u"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpCurrentSel::B10)
    }
    #[doc = "10u"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpCurrentSel::B11)
    }
}
#[doc = "KVCO to control VCO band\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Kvco {
    #[doc = "0: increase KVCO by 20%"]
    B00 = 0,
    #[doc = "1: increase KVCO by 20%"]
    B01 = 1,
    #[doc = "2: increase KVCO by 20%"]
    B10 = 2,
    #[doc = "3: increase KVCO by 20%"]
    B11 = 3,
}
impl From<Kvco> for u8 {
    #[inline(always)]
    fn from(variant: Kvco) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Kvco {
    type Ux = u8;
}
#[doc = "Field `KVCO` reader - KVCO to control VCO band"]
pub type KvcoR = crate::FieldReader<Kvco>;
impl KvcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kvco {
        match self.bits {
            0 => Kvco::B00,
            1 => Kvco::B01,
            2 => Kvco::B10,
            3 => Kvco::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Kvco::B00
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Kvco::B01
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Kvco::B10
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Kvco::B11
    }
}
#[doc = "Field `KVCO` writer - KVCO to control VCO band"]
pub type KvcoW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Kvco>;
impl<'a, REG> KvcoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Kvco::B00)
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Kvco::B01)
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Kvco::B10)
    }
    #[doc = "increase KVCO by 20%"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Kvco::B11)
    }
}
#[doc = "1.5v LDO output voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LdoOutputVSel {
    #[doc = "0: 1.50v"]
    B00 = 0,
    #[doc = "1: 1.50v"]
    B01 = 1,
    #[doc = "2: 1.50v"]
    B10 = 2,
    #[doc = "3: 1.50v"]
    B11 = 3,
}
impl From<LdoOutputVSel> for u8 {
    #[inline(always)]
    fn from(variant: LdoOutputVSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LdoOutputVSel {
    type Ux = u8;
}
#[doc = "Field `LDO_OUTPUT_V_SEL` reader - 1.5v LDO output voltage select"]
pub type LdoOutputVSelR = crate::FieldReader<LdoOutputVSel>;
impl LdoOutputVSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LdoOutputVSel {
        match self.bits {
            0 => LdoOutputVSel::B00,
            1 => LdoOutputVSel::B01,
            2 => LdoOutputVSel::B10,
            3 => LdoOutputVSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == LdoOutputVSel::B00
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == LdoOutputVSel::B01
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == LdoOutputVSel::B10
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == LdoOutputVSel::B11
    }
}
#[doc = "Field `LDO_OUTPUT_V_SEL` writer - 1.5v LDO output voltage select"]
pub type LdoOutputVSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LdoOutputVSel>;
impl<'a, REG> LdoOutputVSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(LdoOutputVSel::B00)
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(LdoOutputVSel::B01)
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(LdoOutputVSel::B10)
    }
    #[doc = "1.50v"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(LdoOutputVSel::B11)
    }
}
impl R {
    #[doc = "Bits 0:1 - v2i current select"]
    #[inline(always)]
    pub fn v2i_current_sel(&self) -> V2iCurrentSelR {
        V2iCurrentSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - charge pump current select"]
    #[inline(always)]
    pub fn chg_pump_current_sel(&self) -> ChgPumpCurrentSelR {
        ChgPumpCurrentSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - KVCO to control VCO band"]
    #[inline(always)]
    pub fn kvco(&self) -> KvcoR {
        KvcoR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 1.5v LDO output voltage select"]
    #[inline(always)]
    pub fn ldo_output_v_sel(&self) -> LdoOutputVSelR {
        LdoOutputVSelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - v2i current select"]
    #[inline(always)]
    #[must_use]
    pub fn v2i_current_sel(&mut self) -> V2iCurrentSelW<PllReg2Spec> {
        V2iCurrentSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - charge pump current select"]
    #[inline(always)]
    #[must_use]
    pub fn chg_pump_current_sel(&mut self) -> ChgPumpCurrentSelW<PllReg2Spec> {
        ChgPumpCurrentSelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - KVCO to control VCO band"]
    #[inline(always)]
    #[must_use]
    pub fn kvco(&mut self) -> KvcoW<PllReg2Spec> {
        KvcoW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 1.5v LDO output voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_output_v_sel(&mut self) -> LdoOutputVSelW<PllReg2Spec> {
        LdoOutputVSelW::new(self, 6)
    }
}
#[doc = "Pll_control_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllReg2Spec;
impl crate::RegisterSpec for PllReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_2::R`](R) reader structure"]
impl crate::Readable for PllReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_reg_2::W`](W) writer structure"]
impl crate::Writable for PllReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets PLL_REG_2 to value 0x11"]
impl crate::Resettable for PllReg2Spec {
    const RESET_VALUE: u32 = 0x11;
}
