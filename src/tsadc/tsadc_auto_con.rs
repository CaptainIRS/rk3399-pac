#[doc = "Register `TSADC_AUTO_CON` reader"]
pub type R = crate::R<TsadcAutoConSpec>;
#[doc = "Register `TSADC_AUTO_CON` writer"]
pub type W = crate::W<TsadcAutoConSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoEn {
    #[doc = "0: TSADC controller works at auto mode"]
    B0 = 0,
    #[doc = "1: TSADC controller works at auto mode"]
    B1 = 1,
}
impl From<AutoEn> for bool {
    #[inline(always)]
    fn from(variant: AutoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_EN` reader - "]
pub type AutoEnR = crate::BitReader<AutoEn>;
impl AutoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoEn {
        match self.bits {
            false => AutoEn::B0,
            true => AutoEn::B1,
        }
    }
    #[doc = "TSADC controller works at auto mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AutoEn::B0
    }
    #[doc = "TSADC controller works at auto mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AutoEn::B1
    }
}
#[doc = "Field `AUTO_EN` writer - "]
pub type AutoEnW<'a, REG> = crate::BitWriter<'a, REG, AutoEn>;
impl<'a, REG> AutoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSADC controller works at auto mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoEn::B0)
    }
    #[doc = "TSADC controller works at auto mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoEn::B1)
    }
}
#[doc = "temperature coefficient\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TsadcQSel {
    #[doc = "0: use(1024 - tsadc_q) as output (negative temperature coefficient) RK3399 is negative temprature coefficient, so please set this bit as 1'b1"]
    B0 = 0,
    #[doc = "1: use(1024 - tsadc_q) as output (negative temperature coefficient) RK3399 is negative temprature coefficient, so please set this bit as 1'b1"]
    B1 = 1,
}
impl From<TsadcQSel> for bool {
    #[inline(always)]
    fn from(variant: TsadcQSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSADC_Q_SEL` reader - temperature coefficient"]
pub type TsadcQSelR = crate::BitReader<TsadcQSel>;
impl TsadcQSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TsadcQSel {
        match self.bits {
            false => TsadcQSel::B0,
            true => TsadcQSel::B1,
        }
    }
    #[doc = "use(1024 - tsadc_q) as output (negative temperature coefficient) RK3399 is negative temprature coefficient, so please set this bit as 1'b1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TsadcQSel::B0
    }
    #[doc = "use(1024 - tsadc_q) as output (negative temperature coefficient) RK3399 is negative temprature coefficient, so please set this bit as 1'b1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TsadcQSel::B1
    }
}
#[doc = "Field `TSADC_Q_SEL` writer - temperature coefficient"]
pub type TsadcQSelW<'a, REG> = crate::BitWriter<'a, REG, TsadcQSel>;
impl<'a, REG> TsadcQSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use(1024 - tsadc_q) as output (negative temperature coefficient) RK3399 is negative temprature coefficient, so please set this bit as 1'b1"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TsadcQSel::B0)
    }
    #[doc = "use(1024 - tsadc_q) as output (negative temperature coefficient) RK3399 is negative temprature coefficient, so please set this bit as 1'b1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TsadcQSel::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src0En {
    #[doc = "0: if the temperature of source 0 is too high , TSHUT will be valid"]
    B0 = 0,
    #[doc = "1: if the temperature of source 0 is too high , TSHUT will be valid"]
    B1 = 1,
}
impl From<Src0En> for bool {
    #[inline(always)]
    fn from(variant: Src0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC0_EN` reader - "]
pub type Src0EnR = crate::BitReader<Src0En>;
impl Src0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src0En {
        match self.bits {
            false => Src0En::B0,
            true => Src0En::B1,
        }
    }
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Src0En::B0
    }
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Src0En::B1
    }
}
#[doc = "Field `SRC0_EN` writer - "]
pub type Src0EnW<'a, REG> = crate::BitWriter<'a, REG, Src0En>;
impl<'a, REG> Src0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0En::B0)
    }
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0En::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src1En {
    #[doc = "0: if the temperature of source 0 is too high , TSHUT will be valid"]
    B0 = 0,
    #[doc = "1: if the temperature of source 0 is too high , TSHUT will be valid"]
    B1 = 1,
}
impl From<Src1En> for bool {
    #[inline(always)]
    fn from(variant: Src1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC1_EN` reader - "]
pub type Src1EnR = crate::BitReader<Src1En>;
impl Src1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src1En {
        match self.bits {
            false => Src1En::B0,
            true => Src1En::B1,
        }
    }
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Src1En::B0
    }
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Src1En::B1
    }
}
#[doc = "Field `SRC1_EN` writer - "]
pub type Src1EnW<'a, REG> = crate::BitWriter<'a, REG, Src1En>;
impl<'a, REG> Src1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1En::B0)
    }
    #[doc = "if the temperature of source 0 is too high , TSHUT will be valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1En::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TshutProlarity {
    #[doc = "0: high active"]
    B0 = 0,
    #[doc = "1: high active"]
    B1 = 1,
}
impl From<TshutProlarity> for bool {
    #[inline(always)]
    fn from(variant: TshutProlarity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSHUT_PROLARITY` reader - "]
pub type TshutProlarityR = crate::BitReader<TshutProlarity>;
impl TshutProlarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TshutProlarity {
        match self.bits {
            false => TshutProlarity::B0,
            true => TshutProlarity::B1,
        }
    }
    #[doc = "high active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TshutProlarity::B0
    }
    #[doc = "high active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TshutProlarity::B1
    }
}
#[doc = "Field `TSHUT_PROLARITY` writer - "]
pub type TshutProlarityW<'a, REG> = crate::BitWriter<'a, REG, TshutProlarity>;
impl<'a, REG> TshutProlarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "high active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TshutProlarity::B0)
    }
    #[doc = "high active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TshutProlarity::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src0LtEn {
    #[doc = "0: enable the low temperature monitor of source 0"]
    B0 = 0,
    #[doc = "1: enable the low temperature monitor of source 0"]
    B1 = 1,
}
impl From<Src0LtEn> for bool {
    #[inline(always)]
    fn from(variant: Src0LtEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC0_LT_EN` reader - "]
pub type Src0LtEnR = crate::BitReader<Src0LtEn>;
impl Src0LtEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src0LtEn {
        match self.bits {
            false => Src0LtEn::B0,
            true => Src0LtEn::B1,
        }
    }
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Src0LtEn::B0
    }
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Src0LtEn::B1
    }
}
#[doc = "Field `SRC0_LT_EN` writer - "]
pub type Src0LtEnW<'a, REG> = crate::BitWriter<'a, REG, Src0LtEn>;
impl<'a, REG> Src0LtEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0LtEn::B0)
    }
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0LtEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src1LtEn {
    #[doc = "0: enable the low temperature monitor of source 0"]
    B0 = 0,
    #[doc = "1: enable the low temperature monitor of source 0"]
    B1 = 1,
}
impl From<Src1LtEn> for bool {
    #[inline(always)]
    fn from(variant: Src1LtEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC1_LT_EN` reader - "]
pub type Src1LtEnR = crate::BitReader<Src1LtEn>;
impl Src1LtEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src1LtEn {
        match self.bits {
            false => Src1LtEn::B0,
            true => Src1LtEn::B1,
        }
    }
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Src1LtEn::B0
    }
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Src1LtEn::B1
    }
}
#[doc = "Field `SRC1_LT_EN` writer - "]
pub type Src1LtEnW<'a, REG> = crate::BitWriter<'a, REG, Src1LtEn>;
impl<'a, REG> Src1LtEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1LtEn::B0)
    }
    #[doc = "enable the low temperature monitor of source 0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1LtEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoStatus {
    #[doc = "0: auto mode in progress."]
    B0 = 0,
    #[doc = "1: auto mode in progress."]
    B1 = 1,
}
impl From<AutoStatus> for bool {
    #[inline(always)]
    fn from(variant: AutoStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_STATUS` reader - "]
pub type AutoStatusR = crate::BitReader<AutoStatus>;
impl AutoStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoStatus {
        match self.bits {
            false => AutoStatus::B0,
            true => AutoStatus::B1,
        }
    }
    #[doc = "auto mode in progress."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AutoStatus::B0
    }
    #[doc = "auto mode in progress."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AutoStatus::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleDlySel {
    #[doc = "0: AUTO_PERIOD_HT is used"]
    B0 = 0,
    #[doc = "1: AUTO_PERIOD_HT is used"]
    B1 = 1,
}
impl From<SampleDlySel> for bool {
    #[inline(always)]
    fn from(variant: SampleDlySel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLE_DLY_SEL` reader - "]
pub type SampleDlySelR = crate::BitReader<SampleDlySel>;
impl SampleDlySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SampleDlySel {
        match self.bits {
            false => SampleDlySel::B0,
            true => SampleDlySel::B1,
        }
    }
    #[doc = "AUTO_PERIOD_HT is used"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SampleDlySel::B0
    }
    #[doc = "AUTO_PERIOD_HT is used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SampleDlySel::B1
    }
}
#[doc = "Field `LAST_TSHUT_2GPIO` reader - last_tshut_2gpio for hardware reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
pub type LastTshut2gpioR = crate::BitReader;
#[doc = "Field `LAST_TSHUT_2GPIO` writer - last_tshut_2gpio for hardware reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
pub type LastTshut2gpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_TSHUT_2CRU` reader - last_tshut_2cru for cru first/second reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
pub type LastTshut2cruR = crate::BitReader;
#[doc = "Field `LAST_TSHUT_2CRU` writer - last_tshut_2cru for cru first/second reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
pub type LastTshut2cruW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn auto_en(&self) -> AutoEnR {
        AutoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - temperature coefficient"]
    #[inline(always)]
    pub fn tsadc_q_sel(&self) -> TsadcQSelR {
        TsadcQSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn src0_en(&self) -> Src0EnR {
        Src0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn src1_en(&self) -> Src1EnR {
        Src1EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tshut_prolarity(&self) -> TshutProlarityR {
        TshutProlarityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn src0_lt_en(&self) -> Src0LtEnR {
        Src0LtEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn src1_lt_en(&self) -> Src1LtEnR {
        Src1LtEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn auto_status(&self) -> AutoStatusR {
        AutoStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sample_dly_sel(&self) -> SampleDlySelR {
        SampleDlySelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - last_tshut_2gpio for hardware reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
    #[inline(always)]
    pub fn last_tshut_2gpio(&self) -> LastTshut2gpioR {
        LastTshut2gpioR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - last_tshut_2cru for cru first/second reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
    #[inline(always)]
    pub fn last_tshut_2cru(&self) -> LastTshut2cruR {
        LastTshut2cruR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn auto_en(&mut self) -> AutoEnW<TsadcAutoConSpec> {
        AutoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - temperature coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_q_sel(&mut self) -> TsadcQSelW<TsadcAutoConSpec> {
        TsadcQSelW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn src0_en(&mut self) -> Src0EnW<TsadcAutoConSpec> {
        Src0EnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn src1_en(&mut self) -> Src1EnW<TsadcAutoConSpec> {
        Src1EnW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tshut_prolarity(&mut self) -> TshutProlarityW<TsadcAutoConSpec> {
        TshutProlarityW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn src0_lt_en(&mut self) -> Src0LtEnW<TsadcAutoConSpec> {
        Src0LtEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn src1_lt_en(&mut self) -> Src1LtEnW<TsadcAutoConSpec> {
        Src1LtEnW::new(self, 13)
    }
    #[doc = "Bit 24 - last_tshut_2gpio for hardware reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
    #[inline(always)]
    #[must_use]
    pub fn last_tshut_2gpio(&mut self) -> LastTshut2gpioW<TsadcAutoConSpec> {
        LastTshut2gpioW::new(self, 24)
    }
    #[doc = "Bit 25 - last_tshut_2cru for cru first/second reset TSHUT status. This bit will set to 1 when tshut is valid, and only be cleared when application write 1 to it. This bit will not be cleared by system reset."]
    #[inline(always)]
    #[must_use]
    pub fn last_tshut_2cru(&mut self) -> LastTshut2cruW<TsadcAutoConSpec> {
        LastTshut2cruW::new(self, 25)
    }
}
#[doc = "TSADC auto mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_auto_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_auto_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcAutoConSpec;
impl crate::RegisterSpec for TsadcAutoConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_auto_con::R`](R) reader structure"]
impl crate::Readable for TsadcAutoConSpec {}
#[doc = "`write(|w| ..)` method takes [`tsadc_auto_con::W`](W) writer structure"]
impl crate::Writable for TsadcAutoConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSADC_AUTO_CON to value 0"]
impl crate::Resettable for TsadcAutoConSpec {
    const RESET_VALUE: u32 = 0;
}
