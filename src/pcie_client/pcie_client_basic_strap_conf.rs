#[doc = "Register `PCIE_CLIENT_BASIC_STRAP_CONF` reader"]
pub type R = crate::R<PcieClientBasicStrapConfSpec>;
#[doc = "Register `PCIE_CLIENT_BASIC_STRAP_CONF` writer"]
pub type W = crate::W<PcieClientBasicStrapConfSpec>;
#[doc = "Config enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfEn {
    #[doc = "0: enable When this input is set to 0 in the EP mode, the core will generate a CRS Completion in response to Configuration Requests. In systems where the core configuration registers are loaded from RAM on power-up, this prevents the core from responding to Configuration Requests before all the registers are loaded. This input can be strapped high when the power-on default values of the Configuration Registers do not need to be modified before Configuration Space enumeration."]
    B0 = 0,
    #[doc = "1: enable When this input is set to 0 in the EP mode, the core will generate a CRS Completion in response to Configuration Requests. In systems where the core configuration registers are loaded from RAM on power-up, this prevents the core from responding to Configuration Requests before all the registers are loaded. This input can be strapped high when the power-on default values of the Configuration Registers do not need to be modified before Configuration Space enumeration."]
    B1 = 1,
}
impl From<ConfEn> for bool {
    #[inline(always)]
    fn from(variant: ConfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONF_EN` reader - Config enable"]
pub type ConfEnR = crate::BitReader<ConfEn>;
impl ConfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConfEn {
        match self.bits {
            false => ConfEn::B0,
            true => ConfEn::B1,
        }
    }
    #[doc = "enable When this input is set to 0 in the EP mode, the core will generate a CRS Completion in response to Configuration Requests. In systems where the core configuration registers are loaded from RAM on power-up, this prevents the core from responding to Configuration Requests before all the registers are loaded. This input can be strapped high when the power-on default values of the Configuration Registers do not need to be modified before Configuration Space enumeration."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ConfEn::B0
    }
    #[doc = "enable When this input is set to 0 in the EP mode, the core will generate a CRS Completion in response to Configuration Requests. In systems where the core configuration registers are loaded from RAM on power-up, this prevents the core from responding to Configuration Requests before all the registers are loaded. This input can be strapped high when the power-on default values of the Configuration Registers do not need to be modified before Configuration Space enumeration."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ConfEn::B1
    }
}
#[doc = "Field `CONF_EN` writer - Config enable"]
pub type ConfEnW<'a, REG> = crate::BitWriter<'a, REG, ConfEn>;
impl<'a, REG> ConfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable When this input is set to 0 in the EP mode, the core will generate a CRS Completion in response to Configuration Requests. In systems where the core configuration registers are loaded from RAM on power-up, this prevents the core from responding to Configuration Requests before all the registers are loaded. This input can be strapped high when the power-on default values of the Configuration Registers do not need to be modified before Configuration Space enumeration."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ConfEn::B0)
    }
    #[doc = "enable When this input is set to 0 in the EP mode, the core will generate a CRS Completion in response to Configuration Requests. In systems where the core configuration registers are loaded from RAM on power-up, this prevents the core from responding to Configuration Requests before all the registers are loaded. This input can be strapped high when the power-on default values of the Configuration Registers do not need to be modified before Configuration Space enumeration."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ConfEn::B1)
    }
}
#[doc = "Link training enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkTrainEn {
    #[doc = "0: enable link training This input must be set to 1 to enable the LTSSM to bring up the link. Setting it to 0 forces the LTSSM to stay in the Detect Quiet state."]
    B0 = 0,
    #[doc = "1: enable link training This input must be set to 1 to enable the LTSSM to bring up the link. Setting it to 0 forces the LTSSM to stay in the Detect Quiet state."]
    B1 = 1,
}
impl From<LinkTrainEn> for bool {
    #[inline(always)]
    fn from(variant: LinkTrainEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_TRAIN_EN` reader - Link training enable"]
pub type LinkTrainEnR = crate::BitReader<LinkTrainEn>;
impl LinkTrainEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkTrainEn {
        match self.bits {
            false => LinkTrainEn::B0,
            true => LinkTrainEn::B1,
        }
    }
    #[doc = "enable link training This input must be set to 1 to enable the LTSSM to bring up the link. Setting it to 0 forces the LTSSM to stay in the Detect Quiet state."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LinkTrainEn::B0
    }
    #[doc = "enable link training This input must be set to 1 to enable the LTSSM to bring up the link. Setting it to 0 forces the LTSSM to stay in the Detect Quiet state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LinkTrainEn::B1
    }
}
#[doc = "Field `LINK_TRAIN_EN` writer - Link training enable"]
pub type LinkTrainEnW<'a, REG> = crate::BitWriter<'a, REG, LinkTrainEn>;
impl<'a, REG> LinkTrainEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable link training This input must be set to 1 to enable the LTSSM to bring up the link. Setting it to 0 forces the LTSSM to stay in the Detect Quiet state."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LinkTrainEn::B0)
    }
    #[doc = "enable link training This input must be set to 1 to enable the LTSSM to bring up the link. Setting it to 0 forces the LTSSM to stay in the Detect Quiet state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LinkTrainEn::B1)
    }
}
#[doc = "Single root I/O virtualization feature enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrIovEn {
    #[doc = "0: enable SR-IOV feature In a core supporting the Single Root I/O Virtualization feature, this strap input must be tied high to enable the SR-IOV feature. The ari_en input must also be strapped high to enable the SR- IOV feature."]
    B0 = 0,
    #[doc = "1: enable SR-IOV feature In a core supporting the Single Root I/O Virtualization feature, this strap input must be tied high to enable the SR-IOV feature. The ari_en input must also be strapped high to enable the SR- IOV feature."]
    B1 = 1,
}
impl From<SrIovEn> for bool {
    #[inline(always)]
    fn from(variant: SrIovEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR_IOV_EN` reader - Single root I/O virtualization feature enable"]
pub type SrIovEnR = crate::BitReader<SrIovEn>;
impl SrIovEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrIovEn {
        match self.bits {
            false => SrIovEn::B0,
            true => SrIovEn::B1,
        }
    }
    #[doc = "enable SR-IOV feature In a core supporting the Single Root I/O Virtualization feature, this strap input must be tied high to enable the SR-IOV feature. The ari_en input must also be strapped high to enable the SR- IOV feature."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SrIovEn::B0
    }
    #[doc = "enable SR-IOV feature In a core supporting the Single Root I/O Virtualization feature, this strap input must be tied high to enable the SR-IOV feature. The ari_en input must also be strapped high to enable the SR- IOV feature."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SrIovEn::B1
    }
}
#[doc = "Field `SR_IOV_EN` writer - Single root I/O virtualization feature enable"]
pub type SrIovEnW<'a, REG> = crate::BitWriter<'a, REG, SrIovEn>;
impl<'a, REG> SrIovEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable SR-IOV feature In a core supporting the Single Root I/O Virtualization feature, this strap input must be tied high to enable the SR-IOV feature. The ari_en input must also be strapped high to enable the SR- IOV feature."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SrIovEn::B0)
    }
    #[doc = "enable SR-IOV feature In a core supporting the Single Root I/O Virtualization feature, this strap input must be tied high to enable the SR-IOV feature. The ari_en input must also be strapped high to enable the SR- IOV feature."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SrIovEn::B1)
    }
}
#[doc = "Alternate interpretation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AriEn {
    #[doc = "0: alternate interpretation of the PCI Routing ID This input is strapped to 0 for legacy interpretation of the PCI Routing ID (8-bit Bus + 5-bit Device + 3- bit Function). A 1 at this input enables the alternate interpretation (8-bit Bus + 8-bit Function)."]
    B0 = 0,
    #[doc = "1: alternate interpretation of the PCI Routing ID This input is strapped to 0 for legacy interpretation of the PCI Routing ID (8-bit Bus + 5-bit Device + 3- bit Function). A 1 at this input enables the alternate interpretation (8-bit Bus + 8-bit Function)."]
    B1 = 1,
}
impl From<AriEn> for bool {
    #[inline(always)]
    fn from(variant: AriEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARI_EN` reader - Alternate interpretation enable"]
pub type AriEnR = crate::BitReader<AriEn>;
impl AriEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AriEn {
        match self.bits {
            false => AriEn::B0,
            true => AriEn::B1,
        }
    }
    #[doc = "alternate interpretation of the PCI Routing ID This input is strapped to 0 for legacy interpretation of the PCI Routing ID (8-bit Bus + 5-bit Device + 3- bit Function). A 1 at this input enables the alternate interpretation (8-bit Bus + 8-bit Function)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AriEn::B0
    }
    #[doc = "alternate interpretation of the PCI Routing ID This input is strapped to 0 for legacy interpretation of the PCI Routing ID (8-bit Bus + 5-bit Device + 3- bit Function). A 1 at this input enables the alternate interpretation (8-bit Bus + 8-bit Function)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AriEn::B1
    }
}
#[doc = "Field `ARI_EN` writer - Alternate interpretation enable"]
pub type AriEnW<'a, REG> = crate::BitWriter<'a, REG, AriEn>;
impl<'a, REG> AriEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "alternate interpretation of the PCI Routing ID This input is strapped to 0 for legacy interpretation of the PCI Routing ID (8-bit Bus + 5-bit Device + 3- bit Function). A 1 at this input enables the alternate interpretation (8-bit Bus + 8-bit Function)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AriEn::B0)
    }
    #[doc = "alternate interpretation of the PCI Routing ID This input is strapped to 0 for legacy interpretation of the PCI Routing ID (8-bit Bus + 5-bit Device + 3- bit Function). A 1 at this input enables the alternate interpretation (8-bit Bus + 8-bit Function)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AriEn::B1)
    }
}
#[doc = "Field `LANE_COUNT_IN` reader - configure the lane count supported 2’b11: reserved 2’b10: X4 2’b01: X2 2’b00: X1"]
pub type LaneCountInR = crate::FieldReader;
#[doc = "Field `LANE_COUNT_IN` writer - configure the lane count supported 2’b11: reserved 2’b10: X4 2’b01: X2 2’b00: X1"]
pub type LaneCountInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Controller operation mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeSelect {
    #[doc = "0: Root Port operation"]
    B0 = 0,
    #[doc = "1: Root Port operation"]
    B1 = 1,
}
impl From<ModeSelect> for bool {
    #[inline(always)]
    fn from(variant: ModeSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE_SELECT` reader - Controller operation mode select"]
pub type ModeSelectR = crate::BitReader<ModeSelect>;
impl ModeSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModeSelect {
        match self.bits {
            false => ModeSelect::B0,
            true => ModeSelect::B1,
        }
    }
    #[doc = "Root Port operation"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ModeSelect::B0
    }
    #[doc = "Root Port operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ModeSelect::B1
    }
}
#[doc = "Field `MODE_SELECT` writer - Controller operation mode select"]
pub type ModeSelectW<'a, REG> = crate::BitWriter<'a, REG, ModeSelect>;
impl<'a, REG> ModeSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Root Port operation"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSelect::B0)
    }
    #[doc = "Root Port operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSelect::B1)
    }
}
#[doc = "Generation support select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieGenSel {
    #[doc = "0: Gen2 mode This strap input selects the generation of the PCI Express protocol supported by the core. If Gen1 mode. The core advertises only Gen1 capability in this mode, and will always operate at Gen1 speed. If Gen2 mode. The core advertises Gen1 and Gen2 capabilities in this mode, but not Gen3. The link may operate at Gen1 or Gen2 speed."]
    B0 = 0,
    #[doc = "1: Gen2 mode This strap input selects the generation of the PCI Express protocol supported by the core. If Gen1 mode. The core advertises only Gen1 capability in this mode, and will always operate at Gen1 speed. If Gen2 mode. The core advertises Gen1 and Gen2 capabilities in this mode, but not Gen3. The link may operate at Gen1 or Gen2 speed."]
    B1 = 1,
}
impl From<PcieGenSel> for bool {
    #[inline(always)]
    fn from(variant: PcieGenSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_GEN_SEL` reader - Generation support select"]
pub type PcieGenSelR = crate::BitReader<PcieGenSel>;
impl PcieGenSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieGenSel {
        match self.bits {
            false => PcieGenSel::B0,
            true => PcieGenSel::B1,
        }
    }
    #[doc = "Gen2 mode This strap input selects the generation of the PCI Express protocol supported by the core. If Gen1 mode. The core advertises only Gen1 capability in this mode, and will always operate at Gen1 speed. If Gen2 mode. The core advertises Gen1 and Gen2 capabilities in this mode, but not Gen3. The link may operate at Gen1 or Gen2 speed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieGenSel::B0
    }
    #[doc = "Gen2 mode This strap input selects the generation of the PCI Express protocol supported by the core. If Gen1 mode. The core advertises only Gen1 capability in this mode, and will always operate at Gen1 speed. If Gen2 mode. The core advertises Gen1 and Gen2 capabilities in this mode, but not Gen3. The link may operate at Gen1 or Gen2 speed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieGenSel::B1
    }
}
#[doc = "Field `PCIE_GEN_SEL` writer - Generation support select"]
pub type PcieGenSelW<'a, REG> = crate::BitWriter<'a, REG, PcieGenSel>;
impl<'a, REG> PcieGenSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Gen2 mode This strap input selects the generation of the PCI Express protocol supported by the core. If Gen1 mode. The core advertises only Gen1 capability in this mode, and will always operate at Gen1 speed. If Gen2 mode. The core advertises Gen1 and Gen2 capabilities in this mode, but not Gen3. The link may operate at Gen1 or Gen2 speed."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieGenSel::B0)
    }
    #[doc = "Gen2 mode This strap input selects the generation of the PCI Express protocol supported by the core. If Gen1 mode. The core advertises only Gen1 capability in this mode, and will always operate at Gen1 speed. If Gen2 mode. The core advertises Gen1 and Gen2 capabilities in this mode, but not Gen3. The link may operate at Gen1 or Gen2 speed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieGenSel::B1)
    }
}
#[doc = "Write mask bits For each served bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WriteMask {
    #[doc = "0: write enable"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u16 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u16;
}
#[doc = "Field `WRITE_MASK` writer - Write mask bits For each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Config enable"]
    #[inline(always)]
    pub fn conf_en(&self) -> ConfEnR {
        ConfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Link training enable"]
    #[inline(always)]
    pub fn link_train_en(&self) -> LinkTrainEnR {
        LinkTrainEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single root I/O virtualization feature enable"]
    #[inline(always)]
    pub fn sr_iov_en(&self) -> SrIovEnR {
        SrIovEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternate interpretation enable"]
    #[inline(always)]
    pub fn ari_en(&self) -> AriEnR {
        AriEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - configure the lane count supported 2’b11: reserved 2’b10: X4 2’b01: X2 2’b00: X1"]
    #[inline(always)]
    pub fn lane_count_in(&self) -> LaneCountInR {
        LaneCountInR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Controller operation mode select"]
    #[inline(always)]
    pub fn mode_select(&self) -> ModeSelectR {
        ModeSelectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generation support select"]
    #[inline(always)]
    pub fn pcie_gen_sel(&self) -> PcieGenSelR {
        PcieGenSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Config enable"]
    #[inline(always)]
    #[must_use]
    pub fn conf_en(&mut self) -> ConfEnW<PcieClientBasicStrapConfSpec> {
        ConfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Link training enable"]
    #[inline(always)]
    #[must_use]
    pub fn link_train_en(&mut self) -> LinkTrainEnW<PcieClientBasicStrapConfSpec> {
        LinkTrainEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Single root I/O virtualization feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sr_iov_en(&mut self) -> SrIovEnW<PcieClientBasicStrapConfSpec> {
        SrIovEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Alternate interpretation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ari_en(&mut self) -> AriEnW<PcieClientBasicStrapConfSpec> {
        AriEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - configure the lane count supported 2’b11: reserved 2’b10: X4 2’b01: X2 2’b00: X1"]
    #[inline(always)]
    #[must_use]
    pub fn lane_count_in(&mut self) -> LaneCountInW<PcieClientBasicStrapConfSpec> {
        LaneCountInW::new(self, 4)
    }
    #[doc = "Bit 6 - Controller operation mode select"]
    #[inline(always)]
    #[must_use]
    pub fn mode_select(&mut self) -> ModeSelectW<PcieClientBasicStrapConfSpec> {
        ModeSelectW::new(self, 6)
    }
    #[doc = "Bit 7 - Generation support select"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_gen_sel(&mut self) -> PcieGenSelW<PcieClientBasicStrapConfSpec> {
        PcieGenSelW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Write mask bits For each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PcieClientBasicStrapConfSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Basic strap configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_basic_strap_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_basic_strap_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientBasicStrapConfSpec;
impl crate::RegisterSpec for PcieClientBasicStrapConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_basic_strap_conf::R`](R) reader structure"]
impl crate::Readable for PcieClientBasicStrapConfSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_basic_strap_conf::W`](W) writer structure"]
impl crate::Writable for PcieClientBasicStrapConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_BASIC_STRAP_CONF to value 0xc1"]
impl crate::Resettable for PcieClientBasicStrapConfSpec {
    const RESET_VALUE: u32 = 0xc1;
}
